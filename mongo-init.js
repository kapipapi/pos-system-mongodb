db.createUser(
    {
        user: "kacper",
        pwd: "kacper",
        roles: [
            {
                role: "readWrite",
                db: "pos"
            }
        ]
    }
);