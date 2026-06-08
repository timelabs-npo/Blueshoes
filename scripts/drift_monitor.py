import os
import socket
import math
import numpy as np
from typing import Dict, List

class FreeBsdDriftMonitor:
    def __init__(self, socket_path: str = "/var/run/bs-edge-agent.sock", threshold: float = 0.45):
        self.socket_path = socket_path
        self.threshold = threshold
        self.ground_truth = {"Person": 0.4, "Project": 0.4, "Metric": 0.2}
        
        # Row order: [Person, Project, Metric]
        self.transition_matrix = np.array([
            [0.5, 0.4, 0.1],
            [0.3, 0.5, 0.2],
            [0.6, 0.1, 0.3]
        ])
        self.state_mapping = {"Person": 0, "Project": 1, "Metric": 2}
        self.observed_buffer = []

    def calculate_kl_divergence(self, p: Dict[str, float], q: Dict[str, float]) -> float:
        kl_div = 0.0
        for key in p:
            q_val = q.get(key, 1e-6)
            kl_div += p[key] * math.log2(p[key] / q_val)
        return kl_div

    def predict_future_distribution(self, current_dist: List[float], steps: int = 2) -> np.ndarray:
        v = np.array(current_dist)
        p_n = np.linalg.matrix_power(self.transition_matrix, steps)
        return v.dot(p_n)

    def audit_buffer(self) -> Dict:
        total = len(self.observed_buffer)
        if total == 0: 
            return {"status": "STABLE", "reason": "Buffer clear"}

        counts = [self.observed_buffer.count(k) for k in self.state_mapping.keys()]
        current_vector = [c / total for c in counts]
        current_dict = {k: current_vector[i] for k, i in self.state_mapping.items()}

        current_kl = self.calculate_kl_divergence(self.ground_truth, current_dict)
        future_vector = self.predict_future_distribution(current_vector, steps=2)
        future_dict = {k: future_vector[i] for k, i in self.state_mapping.items()}
        predicted_kl = self.calculate_kl_divergence(self.ground_truth, future_dict)

        status = "CRITICAL_PROACTIVE_RESET" if predicted_kl > self.threshold else "STABLE"
        
        return {
            "sample_size": total,
            "current_kl_bits": round(current_kl, 4),
            "predicted_kl_2_steps": round(predicted_kl, 4),
            "status": status
        }

    def start_server(self):
        if os.path.exists(self.socket_path):
            os.remove(self.socket_path)

        server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        server.bind(self.socket_path)
        os.chmod(self.socket_path, 0o666) # Allow edge agent binary write privileges
        server.listen(5)
        
        print(f"📡 Sovereign Telemetry Sensor active on {self.socket_path}")
        
        try:
            while True:
                conn, _ = server.accept()
                data_bytes = conn.recv(128)
                if not data_bytes:
                    conn.close()
                    continue
                    
                data = data_bytes.decode('utf-8').strip()
                if data in self.state_mapping:
                    self.observed_buffer.append(data)
                    if len(self.observed_buffer) > 50:
                        self.observed_buffer.pop(0)
                        
                    metrics = self.audit_buffer()
                    print(f"[IPC TELEMETRY] Token: {data} | System Status: {metrics['status']} (Predicted KL: {metrics['predicted_kl_2_steps']})")
                    
                    if metrics['status'] == "CRITICAL_PROACTIVE_RESET":
                        print("⚠️ [AUTOMATED OVERRIDE] Threshold crossed. Issuing context reset signal.")
                conn.close()
        except KeyboardInterrupt:
            print("\nShutting down telemetry server.")
        finally:
            if os.path.exists(self.socket_path):
                os.remove(self.socket_path)

if __name__ == "__main__":
    monitor = FreeBsdDriftMonitor()
    monitor.start_server()
