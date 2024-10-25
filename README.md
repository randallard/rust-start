## todo
- add tracing like [production web app](https://youtu.be/3cA_mk4vdWY?si=wSxfKqbtnrX7oaTF&t=608)
- add config lik [production web app](https://youtu.be/3cA_mk4vdWY?si=8oa1xA2JuWub-0Ev&t=866)
- add error handling like [jeremy chone best practices](https://youtu.be/j-VQCYP7wyw?si=Kz4mqGQ4PaWwo_U5)
- - more error techniques in model errors at tag E06 [rust10x web app](https://github.com:rust10x/rust-web-app)

## informix connection with rust

### installing the csdk

download installclientsdk from IBM "Informix Client SDK 4.50.FC11 Linux x86 64" and place in csdk-installer directory

the following commands build the container with csdk files so they can be used in the rust build

```
docker-compose -f docker-compose.csdk.yml up -d
```
```
docker-compose -f docker-compose.csdk.yml exec csdk-installer bash -c "chmod +x /install/installclientsdk && /install/installclientsdk -i silent -DLICENSE_ACCEPTED=TRUE"
```
then you'll have 
```
docker-compose build
```
    docker-compose up

this should work

