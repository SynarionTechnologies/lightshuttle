![Build](https://github.com/LightShuttle/lightshuttle/actions/workflows/ci.yml/badge.svg)
![Build](https://github.com/LightShuttle/lightshuttle/actions/workflows/docker-publish.yml/badge.svg)
![Docker Image](https://img.shields.io/docker/pulls/synarion/lightshuttle?style=flat-square)

# LightShuttle

[Read in English](README.md)

🚀 LightShuttle est un orchestrateur léger, rapide et auto-hébergeable pour les applications conteneurisées, conçu comme une alternative simple à Kubernetes.

---

## Fonctionnalités

- ⚡ API ultra-rapide basée sur [Axum](https://github.com/tokio-rs/axum)
- 🐳 Contrôle direct du CLI Docker (pas de démon interne)
- 🔥 Open-source, entièrement auto-hébergeable
- 🛠 API REST simple (pas de GraphQL)
- 📈 Métriques prêtes pour Prometheus
- 🧹 Pensé pour les développeurs : déploiement rapide, débogage facile
- 📜 Réponses JSON cohérentes pour les erreurs

---

## Architecture

- **Daemon** (`daemon/`) : serveur central gérant les requêtes API et l'orchestration des conteneurs
- **CLI** (`cli/`) : outil en ligne de commande (en cours)
- **Dashboard** (`dashboard/`) : interface web (prévu)

---

## Prérequis

- Rust (>= 1.76)
- Docker installé et accessible (le CLI `docker` doit fonctionner)
- Linux recommandé (testé sur Debian 12). Les builds Windows sont supportés (testé sur Windows 11)

---

## Développement local

```bash
# Cloner le dépôt
git clone https://github.com/LightShuttle/lightshuttle.git
cd lightshuttle

# Installer les dépendances
cargo install cargo-make

# Compiler et tester
make
```

---

## Déploiement Docker

Vous pouvez exécuter LightShuttle directement avec Docker :

```bash
docker run -d \
  -p 7878:7878 \
  -e BIND_ADDRESS=0.0.0.0:7878 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  synarion/lightshuttle:latest
```

Ou avec Docker Compose :

```bash
docker-compose up -d
```

Assurez-vous que Docker est installé et que `docker.sock` est correctement monté.

---

## Lancer le daemon

```bash
# Compilation
make

# Lancement
cargo run --bin lightshuttle_core
```

Par défaut, l'API est disponible sur [http://127.0.0.1:7878](http://127.0.0.1:7878).

Vous pouvez changer l'adresse par défaut via la variable d'environnement `BIND_ADDRESS` :

```bash
BIND_ADDRESS=0.0.0.0:7878 cargo run --bin lightshuttle_core
```

---

## Feuille de route

- [x] Cycle de vie basique des conteneurs (création, liste, suppression, logs)
- [x] Démarrer/Arrêter des conteneurs
- [x] Recherche de conteneurs
- [x] Support des labels
- [x] Mise à jour/Recréation des conteneurs
- [x] Support des montages de volumes
- [x] Politiques de redémarrage
- [ ] Affinage complet des erreurs (codes de sortie Docker, parsing stderr, etc.)
- [ ] Client CLI (`lightshuttle-cli`)
- [ ] Interface web du dashboard
- [ ] Authentification & RBAC (clés API, rôles)
- [ ] Système de templates (style Helm-light)
- [ ] Limites de ressources (CPU/mémoire)
- [ ] Support des healthchecks (probe + redémarrage en cas d'échec)
- [ ] Conteneurs d'initialisation
- [ ] Sauvegarde/restauration des volumes
- [ ] État persistant (sauvegarder optionnellement la config / les conteneurs sur disque)
- [ ] DNS interne / découverte de services
- [ ] Arrêt gracieux & gestion des signaux

---

## Licence

LightShuttle est distribué sous licence GNU Affero General Public License v3.0 (AGPLv3).
Voir [LICENSE](LICENSE) pour plus de détails.

---

## Site web

Site officiel : [https://www.getlightshuttle.com](https://www.getlightshuttle.com)

---

## Crédits

Développé par **[Pierrick FONQUERNE](https://www.linkedin.com/in/pierrickfonquerne/)**.

