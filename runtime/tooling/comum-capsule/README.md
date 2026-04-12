# comum-capsule

CLI Rust-first para o ciclo minimo de vida de capsulas no monorepo.

Comandos atuais:

- `list`
- `check <capsule-dir>`
- `build <capsule-dir>`
- `verify <capsule-dir>`
- `id <capsule-dir>`
- `inspect <capsule-dir>`
- `resolve <app-dir>`

Arquivos envolvidos:

- `capsule.yaml`: identidade semantica local da capsula
- `capsule.build.json`: metadados gerados a partir do build
- `capsules.yaml`: dependencias capsulares declaradas por um app slice
- `capsules.lock`: resolucao material das capsulas usadas por esse app
