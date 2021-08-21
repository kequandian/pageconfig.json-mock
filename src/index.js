const express = require('express')
const low = require('lowdb')
const FileSync = require('lowdb/adapters/FileSync')

// Create server
const app = express()
app.use(express.json());

// Create database instance and start server
const adapter = new FileSync('db.json')

const db = low(adapter)

// Routes

// GET /posts/:id
app.get('/posts/:id', (req, res) => {
    const post = db.get('posts')
        .find({ id: req.params.id })
        .value()

    res.send(post)
})

// GET /posts
app.get('/posts', (req, res) => {
    const post = db.get('pos·ts')
        .value()

    res.send(post)
})

// POST /posts
app.post('/posts', (req, res) => {
    res.send(db.get('posts')
    .push(req.body)
    .last()
    .assign({ id: Date.now().toString() })
    .write())
})

// DELETE /posts
app.delete('/posts/:id', (req, res) => {
    res.send(db.get('posts')
    .remove({ id: req.params.id })
    .write())
})

// PUT /posts
app.put('/posts', (req, res) => {
    res.send(db.get('posts')
    .find({ id: req.body.id })
    .assign(req.body)
    .write())
})

app.post('/custom', (req, res) => {
    Object.keys(req.body || '').forEach(key => {
        if (key && req.body[key]) {
            db.set(key, req.body[key]).write()
        }
    })
    res.send({ 'code': 200, 'data': req.body })
})

app.post('/custom/:name', (req, res) => {
    const name = req.params.name
    let vals = db.get(name)
    if (vals.findIndex({ "id": parseInt(req.body.id) }).value() === -1) {
        vals.push(req.body).write()
        res.send({ 'code': 200, 'data': req.body })
    } else {
        res.send({ 'code': 500, 'msg': "数据冲突" })
    }

})

app.put('/custom/:name', (req, res) => {
    const name = req.params.name
    const id = parseInt(req.body.id)
    const vals = db.get(name)
    if (vals.findIndex({ "id": id }).value() === -1) {
        res.send({ 'code': 500, 'msg': "源数据不存在" })
    } else {
        vals.find({ "id": id }).assign(req.body).write()
        res.send({ 'code': 200, 'data': req.body })
    }

})

app.get('/custom/:name', (req, res) => {
    const name = req.params.name
    const id = parseInt(req.query.id)
    const o = id ? db.get(name).find({ "id": id }).value() : db.get(name).value()
    res.send({ 'code': 200, 'data': o })
})

app.delete('/custom/:name', (req, res) => {
    const name = req.params.name
    const id = req.query.id
    if (id) {
        db.get(name).remove({ "id": id }).write()
    } else {
        db.get(name).remove().write()
    }
    res.send({ 'code': 200, 'msg': "success" })
})

app.post('/form/:id', (req, res) => {
    const id = parseInt(req.params.id)
    if (db.get('forms').findIndex({ "id": id }).value() === -1) {
        db.get('forms').push({ "id": id, "form": req.body }).write()
    } else {
        db.get('forms').find({ "id": id }).assign({ "id": id, "form": req.body }).write()
    }
    res.send({ 'code': 200, 'data': req.body })
})

app.get('/form', (req, res) => {
    const id = parseInt(req.query.id)
    const o = id ? db.get('forms').find({ "id": id }).value() : db.get('forms').value()
    res.send({ 'code': 200, 'data': o['form'] })
})


app.get('/:name', (req, res) => {
    const id = parseInt(req.query.id)
    const name = req.params.name
    const entity = id ? db.get(name).find({ "id": id }).value() : db.get(name).value()
    res.send({ 'code': 200, 'data': entity })
})

app.listen(3000, '0.0.0.0', () => console.log('listening on port 3000'))
