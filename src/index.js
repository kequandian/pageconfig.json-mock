const express = require('express')
const low = require('lowdb')
const FileSync = require('lowdb/adapters/FileSync')
const cors = require('cors')  // Add this line

// Create server
const app = express()
app.use(cors())  // Add this line to enable CORS for all routes
app.use(express.json());

// Create database instance and start server
const adapter = new FileSync('db.json')

const db = low(adapter)

const successResponse = (data) => {
    return {'code': 200, 'data': data}
}

const errorResponse = (msg) => {
    return {'code': 500, 'msg': msg}
}

const checkPermission = (res) => {
    if (process.env.environment === 'production') {
        res.send(errorResponse("update not allow in production environement!"))
        return false
    }
    return true
}

// GET /posts/:id
app.get('/posts/:id', (req, res) => {
    const post = db.get('posts')
        .find({ id: req.params.id })
        .value()

    res.send(post)
})

// GET /posts, /:name instead
// app.get('/posts', (req, res) => {
//     const post = db.get('posts')
//         .value()
//     res.send(post)
// })


// POST /posts
app.post('/posts', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    res.send(db.get('posts')
    .push(req.body)
    .last()
    .assign({ id: Date.now().toString() })
    .write())
})

// DELETE /posts
app.delete('/posts/:id', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    res.send(db.get('posts')
    .remove({ id: req.params.id })
    .write())
})

// PUT /posts
app.put('/posts', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    res.send(db.get('posts')
    .find({ id: req.body.id })
    .assign(req.body)
    .write())
})


// 表单
app.post('/form/:id', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    const id = parseInt(req.params.id)
    if (db.get('forms').findIndex({ "id": id }).value() === -1) {
        db.get('forms').push({ "id": id, "form": req.body }).write()
    } else {
        db.get('forms').find({ "id": id }).assign({ "id": id, "form": req.body }).write()
    }
    res.send(successResponse(req.body))
})

app.get('/form', (req, res) => {
    const id = parseInt(req.query.id)
    const o = id ? (db.get('forms').find({ "id": id }).value())['form'] : db.get('forms').value()
    res.send(successResponse(o))
})

app.delete('/form/:id', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    res.send(db.get('forms')
    .remove({ id: req.params.id })
    .write())
})

// app.post('/data/:id', (req, res) => {
//     if (!checkPermission(res)) {
//         return
//     }
//     const id = parseInt(req.params.id)
//     if (db.get('dataset').findIndex({ "id": id }).value() === -1) {
//         db.get('dataset').push({ "id": id, "data": req.body }).write()
//     } else {
//         db.get('dataset').find({ "id": id }).assign({ "id": id, "data": req.body }).write()
//     }
//     res.send(successResponse(req.body))
// })

// app.get('/data', (req, res) => {
//     const id = parseInt(req.query.id)
//     const o = id ? (db.get('dataset').find({ "id": id }).value())['data'] : db.get('dataset').value()
//     res.send(successResponse(o))
// })

app.post('/data', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    Object.keys(req.body || '').forEach(key => {
        if (key && req.body[key]) {
            db.set(key, req.body[key]).write()
        }
    })
    res.send(successResponse(req.body))
})

app.post('/data/:name', (req, res) => {
    if (!checkPermission(res)) {
        return
    }
    const name = req.params.name
    let vals = db.get(name)
    if (vals.findIndex({ "id": parseInt(req.body.id) }).value() === -1) {
        vals.push(req.body).write()
        res.send(successResponse(req.body))
    } else {
        res.send(errorResponse("data conflict!"))
    }
})

// TODO, put any data if vals is object
// app.put('/data/:name', (req, res) => {
//     if (!checkPermission(res)) {
//         return
//     }
//     const name = req.params.name
//     const id = parseInt(req.body.id)
//     const vals = db.get(name)
//     if (vals.findIndex({ "id": id }).value() === -1) {
//         res.send(errorResponse("datasource not exists!"))
//     } else {
//         vals.find({ "id": id }).assign(req.body).write()
//         res.send(successResponse(req.body))
//     }
// })

app.get('/data/:name', (req, res) => {
    const name = req.params.name
    const id = parseInt(req.query.id)
    const o = id ? db.get(name).find({ "id": id }).value() : db.get(name).value()
    res.send(successResponse(o))
})

app.delete('/data/:name', (req, res) => {
    const name = req.params.name
    const id = req.query.id
    if (id) {
        db.get(name).remove({ "id": id }).write()
    } else {
        db.get(name).remove().write()
    }
    res.send(successResponse(id))
})


// query any name
app.get('/:name', (req, res) => {       
    const id = parseInt(req.query.id)
    const name = req.params.name
    const entity = id ? db.get(name).find({ "id": id }).value() : db.get(name).value()
    res.send(successResponse(entity))
})

app.listen(3000, '0.0.0.0', () => console.log('listening on port 3000'))
