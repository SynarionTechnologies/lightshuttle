# Sécurité

LightShuttle communique avec le démon Docker via un proxy local. Définissez la variable d'environnement `DOCKER_HOST` à `http://docker-proxy:2375` pour que toutes les commandes Docker passent par ce proxy.

Pour activer l'authentification de l'API HTTP, configurez la variable d'environnement `JWT_SECRET` avec un secret d'au moins 32 caractères. Les jetons sont signés en **HS256** et doivent contenir une revendication `exp` (expiration). Les clients envoient `Authorization: Bearer <token>`.

Exemple :

```bash
export JWT_SECRET=$(openssl rand -hex 32)
```

Génération d'un jeton en Rust :

```rust
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;

#[derive(Serialize)]
struct Claims { sub: String, exp: usize }

let token = encode(
    &Header::default(),
    &Claims { sub: "demo".into(), exp: 1_700_000_000 },
    &EncodingKey::from_secret(std::env::var("JWT_SECRET")?.as_bytes()),
)?;
```

L'exécution du daemon ou du CLI en tant que `root` n'est pas supportée. Si lancé en `root`, le processus se termine immédiatement.

L'image Docker embarque un profil seccomp à `/seccomp.json`. LightShuttle l'utilise lors du lancement des conteneurs pour restreindre les appels système disponibles. Vous pouvez remplacer ce profil en définissant la variable `SECCOMP_PROFILE`.
