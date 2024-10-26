## todo
- add tracing like [production web app](https://youtu.be/3cA_mk4vdWY?si=wSxfKqbtnrX7oaTF&t=608)
- add config lik [production web app](https://youtu.be/3cA_mk4vdWY?si=8oa1xA2JuWub-0Ev&t=866)
- add error handling like [jeremy chone best practices](https://youtu.be/j-VQCYP7wyw?si=Kz4mqGQ4PaWwo_U5)
- - more error techniques in model errors at tag E06 [rust10x web app](https://github.com:rust10x/rust-web-app)

## informix connection with rust

### starting an informix container

(if you have trouble - check out https://github.com/merajabi/informix-tutorial-step-by-step-guide-for-beginners)

```bash
docker run -it --name ifx -h ifx			        \
      -p 9088:9088                                  \
      -p 9089:9089                                  \
      -p 27017:27017                                \ 
      -p 27018:27018                                \ 
      -p 27883:27883                                \ 
      -e LICENSE=accept                             \
      ibmcom/informix-developer-database:latest
```

```powershell
docker run -it --name ifx -h ifx -p 9088:9088 -p 9089:9089 -p 27017:27017 -p 27018:27018 -p 27883:27883 -e LICENSE=accept ibmcom/informix-developer-database:latest
```

### installing the csdk

download installclientsdk from IBM "Informix Client SDK 4.50.FC11 Linux x86 64" and place in csdk-installer directory

the following commands build the container with csdk files so they can be used in the rust build

```
docker-compose -f docker-compose.csdk.yml up -d
```
```
docker-compose -f docker-compose.csdk.yml exec csdk-installer bash -c "chmod +x /install/installclientsdk && /install/installclientsdk -i silent -DLICENSE_ACCEPTED=TRUE"
```
then you'll have csdk files in a csdk directory - those will be picked up by the docker-compose
```
docker-compose build
```
    docker-compose up

this should work

