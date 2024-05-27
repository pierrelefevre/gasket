# gasket
GPU-Accelerated Scalable Kubernetes Elastic Transcoder

## Docker images
Two docker images are built: gasket and gasket-base.
- gasket: The main application image, built from ci.Dockerfile. This contains the application and all its dependencies, including the GPU drivers and FFmpeg.
- gasket-base: A base image containing the gasket binary, but without any dependencies. Intended for use in downstream images, in which the dependencies are installed.




## Development
The goal is to avoid unsafe code and keep the codebase as clean and readable as possible.

### Dependencies
- Rust
- FFmpeg

### Running
Build and run
```bash 
cargo run
```

Listen for changes and run
```bash
cargo watch -x run
```

### Docker
Build and run
```bash
docker build . -t gasket && docker run -p 30303:30303/udp --name gasket gasket
```

## Deploying
### Kubernetes
Deploy to k8s using the provided manifests
```bash
kubectl apply -f k8s/
```
### Docker
Docker compose
```bash
docker compose up
```


## Demo data
There is a hosted folder of video files used for testing, located at https://gsktfs.app.kista.cloud
The files are usually placed in the local repo under `gasket/data/`. For file size concerns, the files are not included in the repo.

