[![Tests](https://github.com/szheng3/rust-new-project-template/actions/workflows/tests.yml/badge.svg)](https://github.com/szheng3/rust-new-project-template/actions/workflows/tests.yml)
[![Clippy](https://github.com/szheng3/rust-new-project-template/actions/workflows/lint.yml/badge.svg)](https://github.com/szheng3/rust-new-project-template/actions/workflows/lint.yml)
[![rust](https://github.com/szheng3/rust-new-project-template/actions/workflows/rust.yml/badge.svg)](https://github.com/szheng3/rust-new-project-template/actions/workflows/rust.yml)
[![publish to Dockerhub](https://github.com/szheng3/rust-new-project-template/actions/workflows/build.yml/badge.svg)](https://github.com/szheng3/rust-new-project-template/actions/workflows/build.yml)

# Weekly Rust Progress Report


## Week 2 Progress

This week, I set up a GitHub CI/CD action pipeline for building, linking, and testing. Additionally, I utilized a Dockerfile to package my Rust services. Furthermore, I deployed the service on Google Cloud Platform using Kubernetes. You can access a demo of the setup at https://apiv2.sszzz.me.
> file is located at [.github/workflows](https://github.com/szheng3/rust-new-project-template/tree/main/.github/workflows)

Here is one of the code snippet from the CI/CD pipeline.
```
name: Rust

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose
```

Here is the code from the Kubernetes deployment file.
```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-ml
  namespace: resume-prod
spec:
  selector:
    matchLabels:
      app: rust-ml
  replicas: 1
  template: # template for the pods
    metadata:
      labels:
        app: rust-ml
    spec:
      containers:
        - name: rust-ml
          imagePullPolicy: Always
          image: szheng3/sz-rust-ml:latest
          ports:
            - containerPort: 8000

          resources:
            requests:
              ephemeral-storage: 10Gi
              cpu: 1250m
              memory: 3Gi

      affinity:
        nodeAffinity:
          requiredDuringSchedulingIgnoredDuringExecution:
            nodeSelectorTerms:
              - matchExpressions:
                  - key: cloud.google.com/gke-spot
                    operator: In
                    values:
                      - "true"

---
apiVersion: v1
kind: Service
metadata:
  name: rust-ml
  namespace: resume-prod
spec:
  # This defines which pods are going to be represented by this Service
  # The service becomes a network endpoint for either other services
  # or maybe external users to connect to (eg browser)
  selector:
    app: rust-ml

  ports:
    - name: http
      port: 80
      targetPort: 8000
  type: ClusterIP
```

Here is the code from the Dockerfile.
```
# Use a Rust base image
FROM rust:latest

# Update the package repository and install dependencies
RUN apt-get update && \
    apt-get install -y software-properties-common python3-dev python3-pip libopenblas-dev libopenmpi-dev

WORKDIR /app

COPY . .


# Build the application
RUN cargo build --release

# Expose the application port
EXPOSE 8000

# Set the command to run when the container starts
#CMD ["./target/release/rust-new-project-template"]
CMD ["cargo", "run", "--release"]

```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [server](https://codevoweb.com/build-a-simple-api-with-rust-and-actix-web/)