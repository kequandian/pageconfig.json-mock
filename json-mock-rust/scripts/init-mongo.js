// MongoDB initialization script for json-mock-rust

// Switch to the json_mock database
db = db.getSiblingDB('json_mock');

// Create collections with indexes
db.createCollection('forms');
db.forms.createIndex({ "id": 1 }, { unique: true });

// Create a sample form configuration
db.forms.insertOne({
    id: 109,
    form: {
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
        unFocusedComponentBorder: false
    }
});

print("Database initialization completed successfully!");
print("Created collection: forms");
print("Inserted sample form data");
