#!/bin/bash
# GCP Infrastructure Provisioning Script for bs-edge-agent
# This script enables necessary APIs and provisions base resources for the 9 selected solutions.

set -e

echo "Starting GCP Infrastructure Provisioning..."

# Ensure gcloud is installed
if ! command -v gcloud &> /dev/null
then
    echo "gcloud could not be found. Please install the Google Cloud SDK."
    exit 1
fi

PROJECT_ID=$(gcloud config get-value project)
if [ -z "$PROJECT_ID" ]; then
    echo "No GCP project configured. Please run 'gcloud config set project <PROJECT_ID>'."
    exit 1
fi
echo "Using project: $PROJECT_ID"

echo "1. Enabling required APIs..."
gcloud services enable \
    run.googleapis.com \
    pubsub.googleapis.com \
    firestore.googleapis.com \
    logging.googleapis.com \
    compute.googleapis.com \
    secretmanager.googleapis.com \
    bigquery.googleapis.com \
    storage-component.googleapis.com \
    container.googleapis.com \
    cloudbuild.googleapis.com

echo "2. Setting up Pub/Sub..."
TOPIC_NAME="edge-telemetry-events"
if ! gcloud pubsub topics describe $TOPIC_NAME &>/dev/null; then
    gcloud pubsub topics create $TOPIC_NAME
    echo "Created Pub/Sub topic: $TOPIC_NAME"
else
    echo "Pub/Sub topic $TOPIC_NAME already exists."
fi

echo "3. Setting up Firestore..."
# Note: Firestore initialization requires an App Engine region or location flag and can only be done once.
# If it fails because it already exists, we ignore the error.
gcloud firestore databases create --location=nam5 --type=firestore-native || echo "Firestore might already be initialized."

echo "4. Setting up Cloud Storage..."
BUCKET_NAME="${PROJECT_ID}-edge-backups"
if ! gcloud storage ls "gs://$BUCKET_NAME" &>/dev/null; then
    gcloud storage buckets create "gs://$BUCKET_NAME" --location=US
    echo "Created Cloud Storage bucket: gs://$BUCKET_NAME"
else
    echo "Cloud Storage bucket gs://$BUCKET_NAME already exists."
fi

echo "5. Setting up BigQuery..."
DATASET_NAME="edge_analytics"
if ! bq show "$PROJECT_ID:$DATASET_NAME" &>/dev/null; then
    bq mk --location=US "$PROJECT_ID:$DATASET_NAME"
    echo "Created BigQuery dataset: $DATASET_NAME"
else
    echo "BigQuery dataset $DATASET_NAME already exists."
fi

echo "6. Setting up Secret Manager..."
SECRET_NAME="router-psk-keys"
if ! gcloud secrets describe $SECRET_NAME &>/dev/null; then
    gcloud secrets create $SECRET_NAME --replication-policy="automatic"
    echo "dummy-key-1234" | gcloud secrets versions add $SECRET_NAME --data-file=-
    echo "Created Secret: $SECRET_NAME with a dummy initial version."
else
    echo "Secret $SECRET_NAME already exists."
fi

echo "6.5 Setting up Cloud Run (Hello World)..."
if ! gcloud run services describe edge-packet-analyzer --region=us-central1 &>/dev/null; then
    gcloud run deploy edge-packet-analyzer \
        --image=gcr.io/cloudrun/hello \
        --region=us-central1 \
        --allow-unauthenticated \
        --quiet
    echo "Deployed Cloud Run service: edge-packet-analyzer"
else
    echo "Cloud Run service edge-packet-analyzer already exists."
fi

echo "6.6 Setting up GKE Autopilot Cluster..."
CLUSTER_NAME="edge-control-plane"
if ! gcloud container clusters describe $CLUSTER_NAME --region=us-central1 &>/dev/null; then
    gcloud container clusters create-auto $CLUSTER_NAME \
        --region=us-central1 \
        --async \
        --quiet
    echo "Started async creation of GKE Autopilot cluster: $CLUSTER_NAME (will take ~10 minutes)"
else
    echo "GKE cluster $CLUSTER_NAME already exists or is creating."
fi

echo "7. Service Account Setup for bs-edge-agent..."
SA_NAME="bs-edge-agent-sa"
SA_EMAIL="${SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"

if ! gcloud iam service-accounts describe $SA_EMAIL &>/dev/null; then
    gcloud iam service-accounts create $SA_NAME --display-name="BS Edge Agent Runtime"
    echo "Created Service Account: $SA_EMAIL"
else
    echo "Service Account $SA_EMAIL already exists."
fi

echo "Binding roles to the Service Account..."
ROLES=(
    "roles/pubsub.publisher"
    "roles/datastore.user"
    "roles/logging.logWriter"
    "roles/secretmanager.secretAccessor"
    "roles/run.invoker"
)

for ROLE in "${ROLES[@]}"; do
    gcloud projects add-iam-policy-binding $PROJECT_ID \
        --member="serviceAccount:$SA_EMAIL" \
        --role="$ROLE" \
        --condition=None > /dev/null
done

echo "Generating Service Account Key..."
if [ ! -f "agent-gcp-key.json" ]; then
    gcloud iam service-accounts keys create "agent-gcp-key.json" --iam-account=$SA_EMAIL
    echo "Saved key to agent-gcp-key.json"
else
    echo "agent-gcp-key.json already exists. Skipping key generation."
fi

echo "Provisioning complete. You can now use agent-gcp-key.json with bs-edge-agent."
