FROM node:21
WORKDIR /usr/src/app
COPY ./reproduction/package.json ./
RUN npm i
COPY ./reproduction/index.js ./
#COPY ./node node_modules/@iota/sdk-wasm/
CMD "node" "index.js"