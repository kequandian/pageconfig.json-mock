// MongoDB initialization script for json-mock-rust

// Switch to the json_mock database
db = db.getSiblingDB('json_mock');

// Create collections with indexes
db.createCollection('posts');
db.posts.createIndex({ "id": 1 }, { unique: true });

db.createCollection('users');
db.users.createIndex({ "id": 1 }, { unique: true });

db.createCollection('forms');
db.forms.createIndex({ "id": 1 }, { unique: true });

db.createCollection('pages');
db.pages.createIndex({ "id": 1 }, { unique: true });

// Insert sample data for testing
db.posts.insertMany([
    {
        id: 1621235592239,
        title: "Sample Post 1",
        name: "Sample Name 1",
        created_at: new Date(),
        updated_at: new Date()
    },
    {
        id: 1621236139288,
        title: "Sample Post 2",
        name: "Sample Name 2",
        created_at: new Date(),
        updated_at: new Date()
    }
]);

db.users.insertMany([
    {
        id: 1,
        name: "Test User 1",
        email: "user1@example.com",
        created_at: new Date(),
        updated_at: new Date()
    },
    {
        id: 2,
        name: "Test User 2",
        email: "user2@example.com",
        created_at: new Date(),
        updated_at: new Date()
    }
]);

// Create a sample form configuration
db.forms.insertOne({
    id: 109,
    fields: [
        {
            "__config__": {
                layout: "rowFormItem",
                tagIcon: "row",
                componentName: "自查事项",
                children: [
                    {
                        "__config__": {
                            label: "自查项 #1",
                            showLabel: true,
                            tag: "el-input",
                            tagIcon: "input",
                            required: false,
                            layout: "colFormItem"
                        },
                        readonly: false,
                        inputBlock: true,
                        placeholder: "请输入自查说明",
                        "__vModel__": "check1"
                    }
                ]
            }
        }
    ],
    formRef: "elForm",
    formModel: "formData",
    size: "medium",
    labelPosition: "left",
    labelWidth: 100,
    formRules: "rules",
    gutter: 0,
    disabled: false,
    formBtns: true,
    unFocusedComponentBorder: false,
    created_at: new Date(),
    updated_at: new Date()
});

print("Database initialization completed successfully!");
print("Created collections: posts, users, forms, pages");
print("Inserted sample data for testing");