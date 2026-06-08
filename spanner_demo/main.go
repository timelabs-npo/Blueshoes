package main

import (
    "context"
    "fmt"
    "os"

    instance "cloud.google.com/go/spanner/admin/instance/apiv1"
    "cloud.google.com/go/spanner/admin/instance/apiv1/instancepb"
    "google.golang.org/api/iterator"
)

func main() {
    // Project ID is taken from the GOOGLE_CLOUD_PROJECT env variable or from the active gcloud config.
    projectID := os.Getenv("GOOGLE_CLOUD_PROJECT")
    if projectID == "" {
        fmt.Println("Error: GOOGLE_CLOUD_PROJECT environment variable not set.")
        os.Exit(1)
    }
    ctx := context.Background()
    client, err := instance.NewInstanceAdminClient(ctx)
    if err != nil {
        fmt.Printf("Failed to create Spanner instance admin client: %v\n", err)
        os.Exit(1)
    }
    defer client.Close()

    // List all instances in the project.
    req := &instancepb.ListInstancesRequest{Parent: fmt.Sprintf("projects/%s", projectID)}
    it := client.ListInstances(ctx, req)
    fmt.Printf("Spanner instances in project %s:\n", projectID)
    for {
        inst, err := it.Next()
        if err != nil {
            if err == iterator.Done {
                break
            }
            fmt.Printf("Error while iterating instances: %v\n", err)
            os.Exit(1)
        }
        fmt.Printf("- ID: %s, Config: %s, NodeCount: %d, State: %s\n", inst.Name, inst.Config, inst.NodeCount, inst.State)
    }
}
