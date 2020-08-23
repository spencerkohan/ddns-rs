# DDNS

DDNS is a tool for automating dynamic DNS configuration using the cloudflare API.

## Prerequisites:

In order for DDNS to work, the following conditions must be met:

### 1. The target systm is accessible to the internet

Dynamic DNS will only work if the target machine can be accessed to the internet.  In the case of a home PC, this will often mean setting up port forwarding on the home router such that the machine can be accessed from outside the LAN.

### 2. The target domain has DNS records hosted by Cloudflare

In order for DDNS to work, you will need a zone for the target domain set up on Cloudflare.

### 3. Cloudflare API key

Additionally, you will need an API key for Cloudflare.  The key should have the following permissions:

```
EDIT Zone.DNS
```
