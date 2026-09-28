# Security

M0 has no authentication, live Discord connection, remote content rendering, update downloader or secret store. Treat it as a research prototype, not a hardened client.

Do not include credentials or private user data in public issues. A dedicated private reporting channel must be configured when the GitHub repository is created and before a connected release. In the meantime report only non-sensitive summaries to the maintainer.

Before a live release: review authentication, permission/cache invalidation, attachment limits, URL handling, dependency licenses and advisories, DAVE integration, crash-report redaction, signed updates and installer trust. Captures remain local and out of version control.
