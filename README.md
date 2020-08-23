# DDNS

DDNS is a tool for automating dynamic DNS configuration using the cloudflare API.

DDNS automates the process of fetching your sytem's public IP, and then creating DNS records using the cloudflare API.  It's intended to be run as a sytemd startup service.

***note***: this project is a work-in-progress.  The basic functionality is implemented, but it is lacking some polish features like meaningful error messages.

## Prerequisites:

In order for DDNS to work, the following conditions must be met:

### 1. The target systm is accessible to the internet

Dynamic DNS will only work if the target machine can be accessed to the internet.  In the case of a home PC, this will often mean setting up port forwarding on the home router such that the machine can be accessed from outside the LAN.

### 2. The target domain has DNS records hosted by Cloudflare

In order for DDNS to work, you will need a zone for the target domain set up on Cloudflare.

Documentation can be found [here](https://support.cloudflare.com/hc/en-us/articles/201720164-Creating-a-Cloudflare-account-and-adding-a-website)

### 3. Cloudflare API key

Additionally, you will need an API key for Cloudflare.  The key should have the following permissions:

```
EDIT Zone.DNS, Zone.Zone
```

API tokens can be set up [here](https://dash.cloudflare.com/profile/api-tokens)

#### 4. Cloudflare Zone ID

You will also need the zone ID for the domain you will be setting up ddns for.

The Zone ID can be found in the "overview" tab for your domain in the cloudflare dashboard, on the right-hand column.

## Installation

The easiest way to install ddns is using `cargo install`.  Just clone this repository:

```
$ git clone https://github.com/spencerkohan/ddns-rs.git
```

Then install using cargo install:

```
$ cargo install --path ./ddns-rs
```

## Usage

To set up ddns, the first step is to configure using your cloudflare credentials.  This can be done like so:

```
$ ddns configure \
    -domain my.cool.domain.com \
    -api-key <cloudflare api key> \
    -zone-id <cloudflare zone ID> \
```

Here the domain should be the domain or subdomain which the system should be accessible from once ddns has been set up.

After configuration, the cloudflare DNS records can be created or updated using the `update` command:

```
$ ddns update
```

This will fetch the current ipv4 and ipv6 addresses for the current machine, and create A and AAAA records respectively on cloudflare.

## Running as a service

DDNS also contains a command to automate setting it up as a systemd service to run at startup.  This will mean the DNS records will automatically be updated if the IP address changes between sessions.

This can be accomplished by the `activate` command.

`activate` must be run as root, since it requires writing a systemd config.  Because `cargo install` installs binaries to the user's home directory, this means it's easiest to run `aactivate` using the absolute path of the ddns executable, which can be found using the `which` command:

```
// get the absolute path
$ which ddns
/home/<user>/.cargo/bin/ddns

// activate
$ sudo /home/<user>/.cargo/bin/ddns activate --user <user>
```

Here the `user` option will specify under which user systemd should run the ddns service.

It's also possible to remove the service using the `deactivate` command:

```
$ sudo /home/<user>/.cargo/bin/ddns deactivate 
```
