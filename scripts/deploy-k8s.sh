#!/bin/bash

# Deploy Genius Game Server to Kubernetes

echo "🚀 Deploying Genius Game Server to Kubernetes..."

# Get the directory where the script is located
SCRIPT_DIR="$(dirname "$0")"
PROJECT_ROOT="$SCRIPT_DIR/.."
K8S_DIR="$PROJECT_ROOT/k8s"

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl is not installed. Please install it first."
    exit 1
fi

# First, create the namespace
echo "📦 Creating namespace..."
kubectl apply -f "$K8S_DIR/namespace.yaml"

# Wait for namespace to be ready
echo "⏳ Waiting for namespace to be ready..."
if ! kubectl wait --for=condition=Active namespace/genius-games --timeout=30s; then
    echo "⚠️  Namespace creation is taking longer than expected. Continuing anyway..."
fi

# Apply configuration first
echo "⚙️  Applying ConfigMap..."
kubectl apply -f "$K8S_DIR/configmap.yaml" -n genius-games

# Apply deployment
echo "🚀 Applying Deployment..."
kubectl apply -f "$K8S_DIR/deployment.yaml" -n genius-games

# Apply services
echo "🔌 Applying Services..."
kubectl apply -f "$K8S_DIR/service.yaml" -n genius-games

# Apply HPA (optional - may fail if metrics-server not installed)
echo "📈 Applying HorizontalPodAutoscaler..."
kubectl apply -f "$K8S_DIR/hpa.yaml" -n genius-games || echo "⚠️  HPA failed - metrics-server may not be installed"

# Apply Ingress (optional - may fail if no ingress controller)
echo "🌐 Applying Ingress..."
kubectl apply -f "$K8S_DIR/ingress.yaml" -n genius-games || echo "⚠️  Ingress failed - ingress controller may not be installed"

# If secrets file exists, apply it
if [ -f "$K8S_DIR/secrets.yaml" ]; then
    echo "🔐 Applying secrets..."
    kubectl apply -f "$K8S_DIR/secrets.yaml" -n genius-games
else
    echo "ℹ️  No secrets.yaml found - using default configuration"
fi

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📊 Checking deployment status..."
kubectl get all -n genius-games

echo ""
echo "⏳ Waiting for deployment to be ready..."
kubectl rollout status deployment/genius-game-server -n genius-games --timeout=60s || true

echo ""
echo "💡 Tips:"
echo "   - To use kustomize (if available): kubectl apply -k $K8S_DIR"
echo "   - To check logs: kubectl logs -n genius-games -l app=genius-game-server"
echo "   - To describe pods: kubectl describe pods -n genius-games"

echo ""
echo "🌐 To access the service:"
echo "   kubectl port-forward -n genius-games service/genius-game-server 8080:8080"
echo ""
echo "Then open http://localhost:8080 in your browser"