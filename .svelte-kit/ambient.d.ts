
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const NVM_INC: string;
	export const scmbDir: string;
	export const MANPATH: string;
	export const GHOSTTY_RESOURCES_DIR: string;
	export const TERM_PROGRAM: string;
	export const npm_package_scripts_tauri: string;
	export const NODE: string;
	export const INIT_CWD: string;
	export const NVM_CD_FLAGS: string;
	export const npm_package_devDependencies_typescript: string;
	export const SHELL: string;
	export const TERM: string;
	export const e9: string;
	export const npm_package_devDependencies_vite: string;
	export const e8: string;
	export const HOMEBREW_REPOSITORY: string;
	export const TMPDIR: string;
	export const e7: string;
	export const e6: string;
	export const ga_auto_remove: string;
	export const TERM_PROGRAM_VERSION: string;
	export const e5: string;
	export const npm_package_scripts_tailwind_watch: string;
	export const e4: string;
	export const npm_package_scripts_dev: string;
	export const GIT_BINARY: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const e3: string;
	export const TAURI_ENV_DEBUG: string;
	export const e2: string;
	export const npm_package_dependencies__tauri_apps_api: string;
	export const e1: string;
	export const npm_package_private: string;
	export const npm_package_devDependencies__sveltejs_kit: string;
	export const npm_package_devDependencies_svelte_preprocess: string;
	export const npm_config_registry: string;
	export const PNPM_HOME: string;
	export const ZSH: string;
	export const NVM_DIR: string;
	export const PHP_INI_SCAN_DIR: string;
	export const USER: string;
	export const LS_COLORS: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const npm_package_dependencies__tailwindcss_vite: string;
	export const COMMAND_MODE: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_package_devDependencies__sveltejs_adapter_static: string;
	export const GIT_REPOS: string;
	export const SSH_AUTH_SOCK: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const npm_package_devDependencies_postcss: string;
	export const npm_package_devDependencies_tslib: string;
	export const npm_execpath: string;
	export const PAGER: string;
	export const TAURI_ENV_PLATFORM: string;
	export const VIRTUAL_ENV_DISABLE_PROMPT: string;
	export const npm_package_devDependencies_svelte: string;
	export const npm_package_devDependencies__tailwindcss_postcss: string;
	export const LSCOLORS: string;
	export const git_status_command: string;
	export const npm_config_frozen_lockfile: string;
	export const PATH: string;
	export const LaunchInstanceID: string;
	export const TAURI_ENV_FAMILY: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const __CFBundleIdentifier: string;
	export const PWD: string;
	export const npm_package_devDependencies_tailwindcss: string;
	export const npm_command: string;
	export const npm_package_scripts_preview: string;
	export const npm_lifecycle_event: string;
	export const LANG: string;
	export const npm_package_name: string;
	export const npm_package_devDependencies__sveltejs_vite_plugin_svelte: string;
	export const npm_package_devDependencies__tsconfig_svelte: string;
	export const npm_package_packageManager: string;
	export const NODE_PATH: string;
	export const npm_package_scripts_build: string;
	export const GIT_REPO_DIR: string;
	export const XPC_FLAGS: string;
	export const TAURI_ENV_ARCH: string;
	export const npm_config_node_gyp: string;
	export const XPC_SERVICE_NAME: string;
	export const git_env_char: string;
	export const git_setup_aliases: string;
	export const npm_package_version: string;
	export const npm_package_devDependencies_autoprefixer: string;
	export const npm_package_devDependencies_daisyui: string;
	export const npm_package_devDependencies_svelte_check: string;
	export const HOME: string;
	export const SHLVL: string;
	export const npm_package_type: string;
	export const GHOSTTY_SHELL_INTEGRATION_NO_SUDO: string;
	export const TERMINFO: string;
	export const HOMEBREW_PREFIX: string;
	export const LESS: string;
	export const LOGNAME: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const npm_package_dependencies_zod: string;
	export const npm_package_devDependencies__tauri_apps_cli: string;
	export const npm_lifecycle_script: string;
	export const XDG_DATA_DIRS: string;
	export const GHOSTTY_BIN_DIR: string;
	export const BUN_INSTALL: string;
	export const NVM_BIN: string;
	export const npm_config_user_agent: string;
	export const HOMEBREW_CELLAR: string;
	export const INFOPATH: string;
	export const npm_package_dependencies_svelte_spa_router: string;
	export const e10: string;
	export const gs_max_changes: string;
	export const _git_cmd: string;
	export const SECURITYSESSIONID: string;
	export const npm_package_scripts_check: string;
	export const COLORTERM: string;
	export const npm_package_scripts_tailwind_build: string;
	export const npm_node_execpath: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		NVM_INC: string;
		scmbDir: string;
		MANPATH: string;
		GHOSTTY_RESOURCES_DIR: string;
		TERM_PROGRAM: string;
		npm_package_scripts_tauri: string;
		NODE: string;
		INIT_CWD: string;
		NVM_CD_FLAGS: string;
		npm_package_devDependencies_typescript: string;
		SHELL: string;
		TERM: string;
		e9: string;
		npm_package_devDependencies_vite: string;
		e8: string;
		HOMEBREW_REPOSITORY: string;
		TMPDIR: string;
		e7: string;
		e6: string;
		ga_auto_remove: string;
		TERM_PROGRAM_VERSION: string;
		e5: string;
		npm_package_scripts_tailwind_watch: string;
		e4: string;
		npm_package_scripts_dev: string;
		GIT_BINARY: string;
		TAURI_CLI_VERBOSITY: string;
		e3: string;
		TAURI_ENV_DEBUG: string;
		e2: string;
		npm_package_dependencies__tauri_apps_api: string;
		e1: string;
		npm_package_private: string;
		npm_package_devDependencies__sveltejs_kit: string;
		npm_package_devDependencies_svelte_preprocess: string;
		npm_config_registry: string;
		PNPM_HOME: string;
		ZSH: string;
		NVM_DIR: string;
		PHP_INI_SCAN_DIR: string;
		USER: string;
		LS_COLORS: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		npm_package_dependencies__tailwindcss_vite: string;
		COMMAND_MODE: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_package_devDependencies__sveltejs_adapter_static: string;
		GIT_REPOS: string;
		SSH_AUTH_SOCK: string;
		__CF_USER_TEXT_ENCODING: string;
		npm_package_devDependencies_postcss: string;
		npm_package_devDependencies_tslib: string;
		npm_execpath: string;
		PAGER: string;
		TAURI_ENV_PLATFORM: string;
		VIRTUAL_ENV_DISABLE_PROMPT: string;
		npm_package_devDependencies_svelte: string;
		npm_package_devDependencies__tailwindcss_postcss: string;
		LSCOLORS: string;
		git_status_command: string;
		npm_config_frozen_lockfile: string;
		PATH: string;
		LaunchInstanceID: string;
		TAURI_ENV_FAMILY: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		__CFBundleIdentifier: string;
		PWD: string;
		npm_package_devDependencies_tailwindcss: string;
		npm_command: string;
		npm_package_scripts_preview: string;
		npm_lifecycle_event: string;
		LANG: string;
		npm_package_name: string;
		npm_package_devDependencies__sveltejs_vite_plugin_svelte: string;
		npm_package_devDependencies__tsconfig_svelte: string;
		npm_package_packageManager: string;
		NODE_PATH: string;
		npm_package_scripts_build: string;
		GIT_REPO_DIR: string;
		XPC_FLAGS: string;
		TAURI_ENV_ARCH: string;
		npm_config_node_gyp: string;
		XPC_SERVICE_NAME: string;
		git_env_char: string;
		git_setup_aliases: string;
		npm_package_version: string;
		npm_package_devDependencies_autoprefixer: string;
		npm_package_devDependencies_daisyui: string;
		npm_package_devDependencies_svelte_check: string;
		HOME: string;
		SHLVL: string;
		npm_package_type: string;
		GHOSTTY_SHELL_INTEGRATION_NO_SUDO: string;
		TERMINFO: string;
		HOMEBREW_PREFIX: string;
		LESS: string;
		LOGNAME: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		npm_package_dependencies_zod: string;
		npm_package_devDependencies__tauri_apps_cli: string;
		npm_lifecycle_script: string;
		XDG_DATA_DIRS: string;
		GHOSTTY_BIN_DIR: string;
		BUN_INSTALL: string;
		NVM_BIN: string;
		npm_config_user_agent: string;
		HOMEBREW_CELLAR: string;
		INFOPATH: string;
		npm_package_dependencies_svelte_spa_router: string;
		e10: string;
		gs_max_changes: string;
		_git_cmd: string;
		SECURITYSESSIONID: string;
		npm_package_scripts_check: string;
		COLORTERM: string;
		npm_package_scripts_tailwind_build: string;
		npm_node_execpath: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
