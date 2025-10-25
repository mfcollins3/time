FROM gcr.io/distroless/base-debian12

# Set environment variables to disable sound and desktop notifications
ENV TIME_NO_SOUND=1
ENV TIME_NO_DESKTOP_NOTIFICATION=1

# Set the working directory
WORKDIR /opt/time

# Copy the time executable and set execute permissions
# The binary will be copied from the artifacts directory during the workflow
COPY --chmod=755 time /opt/time/bin/time

# Set the entrypoint
ENTRYPOINT ["/opt/time/bin/time"]
