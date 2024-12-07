# Sample CRUD rest api in Rust and PostgreSQL

This is a sample app to showcase Keploy integration capabilities using rust and PostgreSQL.

## Setup app
Now that we have Rust and Docker installed, we will setup our application

```bash
git clone https://github.com/keploy/samples-rust
cd samples-rust/rust-crud-
```
## Using Keploy :
Keploy can be installed on Linux directly and on Windows with the help of WSL. Based on your system architecture, install the keploy latest binary release from here:-

#### Linux
1. AMD Architecture
```zsh
curl --silent --location "https://github.com/keploy/keploy/releases/latest/download/keploy_linux_amd64.tar.gz" | tar xz -C /tmp

sudo mkdir -p /usr/local/bin && sudo mv /tmp/keploy /usr/local/bin && keploy
```

<details>
<Summary> 2. ARM Architecture </Summary>


```zsh
curl --silent --location "https://github.com/keploy/keploy/releases/latest/download/keploy_linux_arm64.tar.gz" | tar xz -C /tmp

sudo mkdir -p /usr/local/bin && sudo mv /tmp/keploy /usr/local/bin && keploy
```
</details>

### Let's start the PostgreSQL Instance
Open the root directory path in your terminal and then execute the following command:
```zsh
docker compose up -d db
docker exec -it db psql -U postgres
```
![database setup](image.png)
### Let's run the CRUD app
```zsh
docker compose build
docker compose up rustapp
```
## Capture testcase using Keploy

```bash
keploy record -c "docker compose up rustapp" --container-name rustapp --network-name keploy-network --debug --buildDelay 60
```
![TestRun](images/image2.png)

### Generate testcase

Open Postman or any other tool, or utilize the Postman VSCode extension. Run get, post and put commands in the postman command terminal.
![alt text](image-1.png)

![alt text](image-2.png)

![alt text](image-3.png)
---

### Run the testcases
Once more, open the terminal with the path set to the root directory of the project.

Now, let's execute the deployment in test mode :
```bash
keploy test -c "docker compose up rustapp" --container-name rustapp --network-name keploy-network --delay 10
```

We get the following output in the terminal -

![alt text](image-4.png)
*Voila!! Our testcases has passed 🌟*
