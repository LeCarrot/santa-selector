struct Names String

struct SecretSanta {
    users: Vec<Names>,
}

impl SecretSanta {
    fn new(users: Vec<String>) -> SecretSanta {
        let mut newUsers = vec![];

        for user in users.iter() {
            newUsers.push(user.trim());
        }

        SecretSanta{
            users: newUsers,
        }
    }
}
