## Informix Connection with Rust

This project demonstrates connecting to an Informix database using Rust.

### Required Environment Variables

Before running the application, ensure the following environment variables are set:

```powershell
$env:INFORMIXSERVER = "informix"        # Server name matching sqlhosts entry
$env:DB_HOST = "host.docker.internal"   # Database host (use host.docker.internal for Docker)
$env:DB_PORT = "9088"                   # Database port
$env:DB_NAME = "sysmaster"              # Database name
$env:DB_USER = "informix"               # Database user
$env:DB_PASSWORD = "in4mix"             # Database password
$env:SERVICE_CONFIG_VAR_ONE = "value1"  # Application config variable
$env:SERVICE_CONFIG_VAR_TWO = "value2"  # Application config variable
```

For Linux/Bash environments:
```bash
export INFORMIXSERVER=informix
export DB_HOST=host.docker.internal
export DB_PORT=9088
export DB_NAME=sysmaster
export DB_USER=informix
export DB_PASSWORD=in4mix
export SERVICE_CONFIG_VAR_ONE=value1
export SERVICE_CONFIG_VAR_TWO=value2
```

### Starting an Informix Container

```powershell
docker run -it --name ifx -h ifx -p 9088:9088 -p 9089:9089 -p 27017:27017 -p 27018:27018 -p 27883:27883 -e LICENSE=accept ibmcom/informix-developer-database:latest
```

For Linux environments:
```bash
docker run -it --name ifx -h ifx \
    -p 9088:9088 \
    -p 9089:9089 \
    -p 27017:27017 \
    -p 27018:27018 \
    -p 27883:27883 \
    -e LICENSE=accept \
    ibmcom/informix-developer-database:latest
```

### Installing the CSDK

1. Download "Informix Client SDK 4.50.FC11 Linux x86 64" (installclientsdk) from IBM and place it in the csdk-installer directory

2. Build and install the CSDK:
```powershell
docker-compose -f docker-compose.csdk.yml up -d
docker-compose -f docker-compose.csdk.yml exec csdk-installer bash -c "chmod +x /install/installclientsdk && /install/installclientsdk -i silent -DLICENSE_ACCEPTED=TRUE"
```

3. Build and run the application:
```powershell
docker-compose build
docker-compose up
```

### Testing the Connection

The application includes a connection test that verifies:
1. Creation of the connection object
2. Successful connection to the database
3. Ability to execute a simple query

If successful, you'll see log messages indicating the connection status. Check the logs for any error messages if the connection fails.

### Troubleshooting

1. Verify sqlhosts configuration matches your environment:
```
informix        onsoctcp        host.docker.internal    9088
informix_dr     drsoctcp        ifx                     9087
```

2. Ensure the CSDK is properly installed in the ./csdk directory
3. Check that all environment variables are set correctly
4. Verify the Informix container is running and accessible

## Todo
- add tracing like [production web app](https://youtu.be/3cA_mk4vdWY?si=wSxfKqbtnrX7oaTF&t=608)
- add config like [production web app](https://youtu.be/3cA_mk4vdWY?si=8oa1xA2JuWub-0Ev&t=866)
- add error handling like [jeremy chone best practices](https://youtu.be/j-VQCYP7wyw?si=Kz4mqGQ4PaWwo_U5)
- more error techniques in model errors at tag E06 [rust10x web app](https://github.com:rust10x/rust-web-app)