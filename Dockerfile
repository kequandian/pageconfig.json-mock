# syntax=docker/dockerfile:experimental
# FROM daocloud.io/library/node:12 as build
FROM node:alpine3.12

WORKDIR /usr/src
ADD ./src ./

RUN npm config set registry https://registry.npm.taobao.org
RUN --mount=type=cache,id=node_modules_cache,target=/usr/src/node_modules,rw npm install
CMD [ "npm", "run", "start" ]
