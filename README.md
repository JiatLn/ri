# rni
A rust version ni.

> [ni](https://github.com/antfu/ni) - 💡 Use the right package manager

<br>

<pre>
cargo install <b>rni</b>
</pre>

<a href='https://docs.npmjs.com/cli/v6/commands/npm'>npm</a> · <a href='https://yarnpkg.com'>yarn</a> · <a href='https://pnpm.js.org/en/'>pnpm</a> · <a href='https://bun.sh/'>bun</a>

<br>

### `rni` - install

```bash
rni

# npm install
# yarn install
# pnpm install
# bun install
```

```bash
rni vite

# npm i vite
# yarn add vite
# pnpm add vite
# bun add vite
```

```bash
rni -f

# npm ci
# yarn install --frozen-lockfile
# pnpm i --frozen-lockfile
# bun install --no-save
```

### `rni r` - run

```bash
rni r dev

# npm run dev
# yarn run dev
# pnpm run dev
# bun run dev
```

```bash
rni r

# interactively select the script to run
# supports https://www.npmjs.com/package/npm-scripts-info convention
```



<p align="left">
  <img
    src="./graphs/rni r.gif"
    alt="rni r"
    title="rni r"
  />
</p>

### `rni un` - uninstall

```bash
rni un webpack

# npm uninstall webpack
# yarn remove webpack
# pnpm remove webpack
# bun remove webpack
```

<br>

### How?

**rni** assumes that you work with lockfiles (and you should)

Before it runs, it will detect your `yarn.lock` / `pnpm-lock.yaml` / `package-lock.json` / `bun.lockb` to know current package manager (or `packageManager` field in your packages.json if specified), and runs the [corresponding commands](https://github.com/JiatLn/ri/blob/main/src/agents.rs).

## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
