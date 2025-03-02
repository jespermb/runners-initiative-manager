
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
	export const scmbDir: string;
	export const NVM_INC: string;
	export const COREPACK_ROOT: string;
	export const TERM_PROGRAM: string;
	export const npm_package_scripts_tauri: string;
	export const project_design_dir: string;
	export const NODE: string;
	export const NVM_CD_FLAGS: string;
	export const INIT_CWD: string;
	export const npm_package_devDependencies_typescript: string;
	export const TERM: string;
	export const SHELL: string;
	export const npm_package_devDependencies_vite: string;
	export const TMPDIR: string;
	export const HOMEBREW_REPOSITORY: string;
	export const ga_auto_remove: string;
	export const design_ext_dirs: string;
	export const TERM_PROGRAM_VERSION: string;
	export const npm_package_scripts_tailwind_watch: string;
	export const npm_package_scripts_dev: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const GIT_BINARY: string;
	export const TERM_SESSION_ID: string;
	export const TAURI_ENV_DEBUG: string;
	export const npm_package_dependencies__tauri_apps_api: string;
	export const npm_package_private: string;
	export const npm_package_devDependencies__sveltejs_kit: string;
	export const npm_package_devDependencies_svelte_preprocess: string;
	export const npm_config_registry: string;
	export const ZSH: string;
	export const PNPM_HOME: string;
	export const USER: string;
	export const PHP_INI_SCAN_DIR: string;
	export const NVM_DIR: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const LS_COLORS: string;
	export const npm_package_dependencies__tailwindcss_vite: string;
	export const COMMAND_MODE: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_package_devDependencies__sveltejs_adapter_static: string;
	export const SSH_AUTH_SOCK: string;
	export const GIT_REPOS: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const npm_package_devDependencies_postcss: string;
	export const TERM_FEATURES: string;
	export const npm_package_devDependencies_tslib: string;
	export const npm_execpath: string;
	export const design_base_dirs: string;
	export const VIRTUAL_ENV_DISABLE_PROMPT: string;
	export const TAURI_ENV_PLATFORM: string;
	export const PAGER: string;
	export const npm_package_devDependencies_svelte: string;
	export const npm_package_devDependencies__tailwindcss_postcss: string;
	export const git_status_command: string;
	export const LSCOLORS: string;
	export const npm_config_frozen_lockfile: string;
	export const root_design_dir: string;
	export const TERMINFO_DIRS: string;
	export const PATH: string;
	export const LaunchInstanceID: string;
	export const __CFBundleIdentifier: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const TAURI_ENV_FAMILY: string;
	export const COREPACK_ENABLE_DOWNLOAD_PROMPT: string;
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
	export const ITERM_PROFILE: string;
	export const NODE_PATH: string;
	export const npm_package_scripts_build: string;
	export const design_av_dirs: string;
	export const XPC_FLAGS: string;
	export const GIT_REPO_DIR: string;
	export const TAURI_ENV_ARCH: string;
	export const npm_config_node_gyp: string;
	export const git_setup_aliases: string;
	export const git_env_char: string;
	export const XPC_SERVICE_NAME: string;
	export const npm_package_version: string;
	export const npm_package_devDependencies_autoprefixer: string;
	export const npm_package_devDependencies_daisyui: string;
	export const npm_package_devDependencies_svelte_check: string;
	export const SHLVL: string;
	export const HOME: string;
	export const COLORFGBG: string;
	export const npm_package_type: string;
	export const LC_TERMINAL_VERSION: string;
	export const HOMEBREW_PREFIX: string;
	export const ITERM_SESSION_ID: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const LOGNAME: string;
	export const LESS: string;
	export const npm_package_dependencies_zod: string;
	export const npm_package_devDependencies__tauri_apps_cli: string;
	export const npm_lifecycle_script: string;
	export const NVM_BIN: string;
	export const BUN_INSTALL: string;
	export const npm_config_user_agent: string;
	export const TAURI_SHELL_PLUGIN_CONFIG: string;
	export const INFOPATH: string;
	export const HOMEBREW_CELLAR: string;
	export const npm_package_dependencies_svelte_spa_router: string;
	export const LC_TERMINAL: string;
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
		scmbDir: string;
		NVM_INC: string;
		COREPACK_ROOT: string;
		TERM_PROGRAM: string;
		npm_package_scripts_tauri: string;
		project_design_dir: string;
		NODE: string;
		NVM_CD_FLAGS: string;
		INIT_CWD: string;
		npm_package_devDependencies_typescript: string;
		TERM: string;
		SHELL: string;
		npm_package_devDependencies_vite: string;
		TMPDIR: string;
		HOMEBREW_REPOSITORY: string;
		ga_auto_remove: string;
		design_ext_dirs: string;
		TERM_PROGRAM_VERSION: string;
		npm_package_scripts_tailwind_watch: string;
		npm_package_scripts_dev: string;
		TAURI_CLI_VERBOSITY: string;
		GIT_BINARY: string;
		TERM_SESSION_ID: string;
		TAURI_ENV_DEBUG: string;
		npm_package_dependencies__tauri_apps_api: string;
		npm_package_private: string;
		npm_package_devDependencies__sveltejs_kit: string;
		npm_package_devDependencies_svelte_preprocess: string;
		npm_config_registry: string;
		ZSH: string;
		PNPM_HOME: string;
		USER: string;
		PHP_INI_SCAN_DIR: string;
		NVM_DIR: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		LS_COLORS: string;
		npm_package_dependencies__tailwindcss_vite: string;
		COMMAND_MODE: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_package_devDependencies__sveltejs_adapter_static: string;
		SSH_AUTH_SOCK: string;
		GIT_REPOS: string;
		__CF_USER_TEXT_ENCODING: string;
		npm_package_devDependencies_postcss: string;
		TERM_FEATURES: string;
		npm_package_devDependencies_tslib: string;
		npm_execpath: string;
		design_base_dirs: string;
		VIRTUAL_ENV_DISABLE_PROMPT: string;
		TAURI_ENV_PLATFORM: string;
		PAGER: string;
		npm_package_devDependencies_svelte: string;
		npm_package_devDependencies__tailwindcss_postcss: string;
		git_status_command: string;
		LSCOLORS: string;
		npm_config_frozen_lockfile: string;
		root_design_dir: string;
		TERMINFO_DIRS: string;
		PATH: string;
		LaunchInstanceID: string;
		__CFBundleIdentifier: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		TAURI_ENV_FAMILY: string;
		COREPACK_ENABLE_DOWNLOAD_PROMPT: string;
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
		ITERM_PROFILE: string;
		NODE_PATH: string;
		npm_package_scripts_build: string;
		design_av_dirs: string;
		XPC_FLAGS: string;
		GIT_REPO_DIR: string;
		TAURI_ENV_ARCH: string;
		npm_config_node_gyp: string;
		git_setup_aliases: string;
		git_env_char: string;
		XPC_SERVICE_NAME: string;
		npm_package_version: string;
		npm_package_devDependencies_autoprefixer: string;
		npm_package_devDependencies_daisyui: string;
		npm_package_devDependencies_svelte_check: string;
		SHLVL: string;
		HOME: string;
		COLORFGBG: string;
		npm_package_type: string;
		LC_TERMINAL_VERSION: string;
		HOMEBREW_PREFIX: string;
		ITERM_SESSION_ID: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		LOGNAME: string;
		LESS: string;
		npm_package_dependencies_zod: string;
		npm_package_devDependencies__tauri_apps_cli: string;
		npm_lifecycle_script: string;
		NVM_BIN: string;
		BUN_INSTALL: string;
		npm_config_user_agent: string;
		TAURI_SHELL_PLUGIN_CONFIG: string;
		INFOPATH: string;
		HOMEBREW_CELLAR: string;
		npm_package_dependencies_svelte_spa_router: string;
		LC_TERMINAL: string;
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
