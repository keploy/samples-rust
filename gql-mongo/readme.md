# Sample app with Rust and MongoDb

This is a sample app to test Keploy integration capabilities using rust and MongoDb.

## Setup app
Now that we have bun installed, we will setup our application

```bash
git clone https://github.com/keploy/samples-rust 
cd samples-rust/gql-mongo
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

### Let's start the MongoDB Instance
```zsh
docker-compose up -d
```
## Capture testcase

```bash
sudo -E env PATH=$PATH keploy record -c 'cargo run'
```

### Generate testcase

Go to the http://127.0.0.1:8080 and create some queries. 

We will get the following output in our terminal

![Test-case](image-1.png)

---

### Run the testcases
Now, let's run the keploy in test mode again:-

```bash
sudo -E env PATH=$PATH keploy test -c 'cargo run'
```

![TestRun](image.png)
*Voila!! Our testcases has passed 🌟*