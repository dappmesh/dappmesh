kubectl create secret docker-registry docker-registry-secret \
  --docker-server=docker-registry.registry.svc.cluster.local:5000 \
  --docker-username=dappmesh \
  --docker-password=dappmesh \
  --namespace=dappmesh \
  --dry-run=client -o yaml > ./manifests/platform/secret/docker-registry.yaml