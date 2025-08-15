# Security

LightShuttle communicates with the Docker daemon through a local proxy. Set the `DOCKER_HOST`
environment variable to `http://docker-proxy:2375` so all Docker commands go through the proxy.

Running the daemon or CLI as `root` is not supported. If launched as `root`, the process exits
immediately.

The Docker image bundles a seccomp profile at `/seccomp.json`. LightShuttle uses it when
spawning containers to restrict available system calls. Override the profile path by setting the
`SECCOMP_PROFILE` environment variable.
