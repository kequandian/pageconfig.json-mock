const express = require('express')
const low = require('lowdb')
const FileAsync = require('lowdb/adapters/FileAsync')

// Create server
const app = express()
app.use(express.json());

// Create database instance and start server
const adapter = new FileAsync('db.json')
low(adapter)
    .then(db => {
        // Routes
        app.get('/:name', (req, res) => {
            const entity = db.get(req.params.name).value()
            res.send(entity)
        })

        // GET /posts/:id
        app.get('/posts/:id', (req, res) => {
            const post = db.get('posts')
                .find({ id: req.params.id })
                .value()

            res.send(post)
        })

        // GET /posts
        app.get('/posts', (req, res) => {
            const post = db.get('posts')
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

        // DELETE /posts
        app.delete('/posts/:id', (req, res) => {
            db.get('posts')
                .remove({ id: req.params.id })
                .write()
                .then(post => res.send(post))
        })

        // PUT /posts
        app.put('/posts', (req, res) => {
            db.get('posts')
                .find({ id: req.body.id })
                .assign(req.body)
                .write()
                .then(post => res.send(post))
        })

        app.post('/custom', (req, res) => {
            const key = req.body.key;
            const val = req.body.value;
            if (key && val) {
                db.set(key, val).write().then(() => res.send(val))
            }
        })

        // Set db default values
        return db.defaults({ posts: [] }).write()
    })
    .then(() => {
        app.listen(3000, '0.0.0.0', () => console.log('listening on port 3000'))
    })
