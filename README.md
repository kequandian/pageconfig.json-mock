## 简介

`lowdb`是一个本地化轻量级存储器，内部也搭载了服务可快速启动，且提供了丰富的API

## 快速启动

>Tips：如若手动更改了`db.json`文件，则需手动重启服务重新载入

```shell
$ npm install
# 其中本项目下的db.json为主存储文件
$ npm run start
```

## API列表

- 检查是否存在

```javascript
db.has('posts')
  .value()
```

- 直接设置

```
db.set('posts', [])
  .write()
```

- 根据访问量统计前五个

```
db.get('posts')
  .filter({published: true})
  .sortBy('views')
  .take(5)
  .value()
```

- 仅获取`title`列表

```
db.get('posts')
  .map('title')
  .value()
```

- 获取总数量

```
db.get('posts')
  .size()
  .value()
```

- 通过指定`title`

```
db.get('posts[0].title')
  .value()
```

- 更新

```
db.get('posts')
  .find({ title: 'low!' })
  .assign({ title: 'hi!'})
  .write()
```

- 删除

```
db.get('posts')
  .remove({ title: 'low!' })
  .write()
```

- 删除指定属性

```
db.unset('user.name')
  .write()
```

- 内部深克隆

```
db.get('posts')
  .cloneDeep()
  .value()
```

## 使用范例

### CLI

```javascript
const low = require('lowdb')
const FileSync = require('lowdb/adapters/FileSync')

const adapter = new FileSync('db.json')
const db = low(adapter)

db.defaults({ posts: [] })
  .write()

const result = db.get('posts')
  .push({ title: process.argv[2] })
  .write()

console.log(result)
$ node cli.js hello
# [ { title: 'hello' } ]
```

### Browser

```shell
import low from 'lowdb'
import LocalStorage from 'lowdb/adapters/LocalStorage'

const adapter = new LocalStorage('db')
const db = low(adapter)

db.defaults({ posts: [] })
  .write()

// Data is automatically saved to localStorage
db.get('posts')
  .push({ title: 'lowdb' })
  .write()

```

### Server

>**Tips：如果正在开发本地服务器且不希望收到并发请求，则使用`file-sync`存储通常会更容易，这也是默认设置，但如果需要避免阻塞请求，则可以使用`file-async`存储**

```javascript
const express = require('express')
const bodyParser = require('body-parser')
const low = require('lowdb')
const FileAsync = require('lowdb/adapters/FileAsync')

// Create server
const app = express()
app.use(bodyParser.json())

// Create database instance and start server
const adapter = new FileAsync('db.json')
low(adapter)
  .then(db => {
    // Routes
    // GET /posts/:id
    app.get('/posts/:id', (req, res) => {
      const post = db.get('posts')
        .find({ id: req.params.id })
        .value()

      res.send(post)
    })

    // POST /posts
    app.post('/posts', (req, res) => {
      db.get('posts')
        .push(req.body)
        .last()
        .assign({ id: Date.now().toString() })
        .write()
        .then(post => res.send(post))
    })

    // Set db default values
    return db.defaults({ posts: [] }).write()
  })
  .then(() => {
    app.listen(3000, () => console.log('listening on port 3000'))
  })
```

### In-memory

```javascript
const fs = require('fs')
const low = require('lowdb')
const FileSync = require('lowdb/adapters/FileSync')
const Memory = require('lowdb/adapters/Memory')

const db = low(
  process.env.NODE_ENV === 'test'
    ? new Memory()
    : new FileSync('db.json')
)

db.defaults({ posts: [] })
  .write()

db.get('posts')
  .push({ title: 'lowdb' })
  .write()
```

