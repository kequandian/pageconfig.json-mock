##
[{JSON} Placeholder](https://jsonplaceholder.typicode.com/)

## 快速启动

> Tips：如若手动更改了`db.json`文件，则需手动重启服务重新载入

- 手动运行

```shell
$ cd src
$ npm install
# 其中本项目下的db.json为主存储文件
$ npm run start
```

- docker 运行

```shell
$ sh startup.sh
```

## 数据接口

- POST `/custom` 「用于自定义编辑数据」

```json
{
  "key1": "val1",
  "key2": "val2"
}
```

- GET `/:key`「用于获取数据库字段」
  - 其中`key`可以通过`user.name`或`list[0].data`等方式进行获取
