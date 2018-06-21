The purpose of r53up is to bind a fixed DNS subdomain to an AWS EC2 instance which is started on-demand
and does not have a fixed public IP address.

It has a simple command-line format: `r53up hostname domain`

e.g: `r53up myhost mydomain.com` will bind `myhost.mydomain.com` to EC2 instance which ran the command.

It is designed to be run automatically at startup and it has a short DNS time-to-live (60 seconds) so that
the host's new IP address is available fairly quickly after an instance is started.
