# Changelog

All notable changes to this project will be documented in this file.

## [0.0.1] - Draft

Initial public milestone for Yoin as an embeddable comment system with a lightweight admin panel.

### Highlights

- Embeddable client comment UI with listing, pagination, nested replies, and basic sorting.
- Comment submission flow with login/register modal and stored user identity support.
- Admin panel for site management, comment moderation actions, and read-only RBAC visibility.
- Runtime config cleanup and clearer frontend module boundaries for `admin`, `client`, `config`, and `shared`.

### Included In This Draft

- Comment creation, reply creation, comment listing, and reply loading.
- Basic authentication flow for client-side login and registration.
- Admin site CRUD flow and admin comment status updates.
- Shared API base configuration and frontend build integration.

### Not Yet Considered Complete

- External identity exchange (`POST /api/auth/external/exchange`) is still unimplemented.
- Email notifications and 2FA are still unimplemented.
- Akismet moderation still parks comments as pending.
- Some admin tabs remain incomplete; OAuth and comment moderation now have a working path.

## [unreleased]

### 🚀 Features

- Add default jwt key function - ([e3d17b3](https://github.com/JQiue/yoin/commit/e3d17b3507330425766e2b6bb5c7608c07cc4f47))
- Implement the API endpoint for create a new comment - ([bf2a515](https://github.com/JQiue/yoin/commit/bf2a515ca1e6b57f0b41e77a0ce344511618f8b5))
- Add site update functionality and error handling - ([11246cb](https://github.com/JQiue/yoin/commit/11246cb1dbb0bda94e75c0bc7f3e1a759f84f5ee))
- Add comment listing and pagination logic - ([cbaca33](https://github.com/JQiue/yoin/commit/cbaca33661e4afc6ee2b24367351c13cb91d9a75))
- Integrate Tailwind CSS and set up comment system - ([fe8df86](https://github.com/JQiue/yoin/commit/fe8df86e13ef75e0abc6f369e0a59b462ad9f2a7))
- Integrate UI build process into Rust build script - ([9530309](https://github.com/JQiue/yoin/commit/9530309e4d774f259f5310dbcd3de50a13ab8bbd))
- Enhance comment structure with thread and parent IDs, update related repository and service logic - ([9f69aaf](https://github.com/JQiue/yoin/commit/9f69aaf724ba7aecfe22836850690347f8b9d7bc))
- Add list_replies function and corresponding repository method for fetching comment replies - ([bcd3885](https://github.com/JQiue/yoin/commit/bcd38853d5d33106899926ae236b12d897ffc77d))
- Add comment reply functionality with UI and state management - ([06fe10c](https://github.com/JQiue/yoin/commit/06fe10cf3d2e35f794120e8b51c5dced10829d08))
- Add biome configuration - ([ed1b0ca](https://github.com/JQiue/yoin/commit/ed1b0ca8e634c2bc7244341bf57331a2e5ac769b))
- Add api_base config support - ([bc4e116](https://github.com/JQiue/yoin/commit/bc4e116b1247a65668c650272cd4df00ceb9dd11))
- Add user identities table - ([49220a5](https://github.com/JQiue/yoin/commit/49220a5957e17006767a5bc0c7e243860a8e762d))
- Add oauth providers table - ([a10b110](https://github.com/JQiue/yoin/commit/a10b110dd285219cf34cf2ad87955171615d65b8))
- Add reactions table - ([435e447](https://github.com/JQiue/yoin/commit/435e447afafad82d7ee6e5a8860abcc88bbd55d9))
- Add comment subscriptions, roles, role permissions, and permissions tables - ([ebc59e5](https://github.com/JQiue/yoin/commit/ebc59e51eb199c236ed9fab7670fe2a076d05b36))
- Add OAuth service implementation - ([7510349](https://github.com/JQiue/yoin/commit/7510349e65801e532df4a26cb9ae9faf13b26ca6))
- Add oauth provider configuration and flow - ([8bb0f8d](https://github.com/JQiue/yoin/commit/8bb0f8d117023e9ea4fe0c48c720da31f58a7cd6))
- Update comment status handling and implement soft delete functionality - ([f22377a](https://github.com/JQiue/yoin/commit/f22377a8709a45c70a78b92b6afde3365135eecc))
- Add soft delete thread functionality - ([e758998](https://github.com/JQiue/yoin/commit/e7589981b438b91f1134d5786883e4ea9cce6f02))
- Add entities for reactions, moderation, and subscriptions - ([762d52d](https://github.com/JQiue/yoin/commit/762d52d7273f8f3afc14be69df57c43dbfe71b09))
- Implement pluggable comment moderation system - ([22a79e0](https://github.com/JQiue/yoin/commit/22a79e0296054e456ed76621311dbbca2563ea99))
- Introduce RBAC bootstrap logic for system initialization - ([9af86f3](https://github.com/JQiue/yoin/commit/9af86f3f351adb8a3d261d7247d799ab2b607578))
- Add oauth provider repository methods - ([ee33e42](https://github.com/JQiue/yoin/commit/ee33e42cce5c5fc6648c42de0b179476d13525df))
- Add moderation provider repository - ([a42b2bb](https://github.com/JQiue/yoin/commit/a42b2bb9a8653b5b17a787b0b69307462b5c2a9b))
- Add comment moderation support - ([d369bf0](https://github.com/JQiue/yoin/commit/d369bf08f00bf5737d8aa3bc3935938f9bedf4cc))
- Add rbac permission management - ([5285b84](https://github.com/JQiue/yoin/commit/5285b84bd3ce641a050fa7aa0d85772cd07bb1a0))
- Refactor comment components structure - ([84d7d45](https://github.com/JQiue/yoin/commit/84d7d45073d4020cd89c021fa8f582675da99704))
- Implement moderation, reaction, and subscription systems - ([9634b87](https://github.com/JQiue/yoin/commit/9634b87eb3c3bd3c1ef894061a2aa7988154faf8))
- Add js handler for client and admin - ([57fa655](https://github.com/JQiue/yoin/commit/57fa65546c95889535bca77051ed3a1dd1180782))
- Add MIT License - ([0f35623](https://github.com/JQiue/yoin/commit/0f35623ec4d6a7e0c253185abe38fa360c3cf043))
- Add find_pending and list_pending_comments methods for comment moderation - ([da21eb6](https://github.com/JQiue/yoin/commit/da21eb69a89a0f70a02981d07e3aaacd147f4cf0))
- Update dependencies and add serde support for enums - ([6571b06](https://github.com/JQiue/yoin/commit/6571b0634d6118b436d25ac6a4e185c0d09ed2d2))
- Implement pagination and sorting for comments retrieval - ([37cc4bd](https://github.com/JQiue/yoin/commit/37cc4bd1658c43e0e09b1a9673cc0e7c4a74a358))
- Add admin moderation endpoints - ([8c15989](https://github.com/JQiue/yoin/commit/8c1598974d699627f740cb8de083a1df6915b028))
- Add admin panel features - ([33a0b8d](https://github.com/JQiue/yoin/commit/33a0b8dc2e5c0c8a3d55df43e7d6ccf2c44b91f6))

### 🐛 Bug Fixes

- Update comment model fields - ([87df921](https://github.com/JQiue/yoin/commit/87df9216c7f97012f7e4f69fd25447364891128a))
- Handle unwrap and add error context in database operations - ([c92bd42](https://github.com/JQiue/yoin/commit/c92bd4291e9bd0d6c2e7a3e4a03415b3db0a1e68))
- Rename page_page to page_path and refactor site config fetching - ([30bc868](https://github.com/JQiue/yoin/commit/30bc868ef1cf71278d25c9a39bbbe709e605cce7))
- Rename total_page to total_pages for consistency in pagination responses - ([e60c0e7](https://github.com/JQiue/yoin/commit/e60c0e7316858af4e3f1e519db423eb1af6c5b16))
- Ensure jwt key meets minimum length requirement - ([b91bb70](https://github.com/JQiue/yoin/commit/b91bb70cefb71e9db8cd9ee224fa2c0d530a929d))
- Trust proxy logic updated - ([ff32c06](https://github.com/JQiue/yoin/commit/ff32c06e3b4b0ded85b851a012ff01c860e40fa9))
- Align form fields with backend API changes - ([28e1d3b](https://github.com/JQiue/yoin/commit/28e1d3b62340c87dba673a00b940a8995df285fd))
- IsLoading might freeze when the comment list request fails - ([ad23e8d](https://github.com/JQiue/yoin/commit/ad23e8d6f51020480ae9b917f4f5af1c39b4ef14))
- Handle website link safely - ([dbaf8fb](https://github.com/JQiue/yoin/commit/dbaf8fbf0b0df16ca6666f231e8fbeb66e41d424))
- Rename OauthProviders to OAuthProviders - ([1e84ac1](https://github.com/JQiue/yoin/commit/1e84ac1b89b7d14f9dfe98a5a80198fa2566fbb6))
- Rename OauthProviders to OAuthProviders for consistency - ([fb28ed4](https://github.com/JQiue/yoin/commit/fb28ed4e6443534429665fe8e98c8a87ce1d4a43))
- Ensure rerun triggers on UI file changes - ([b326c5c](https://github.com/JQiue/yoin/commit/b326c5c77cac71f917668c20fba5f0f318ac164e))

### 🚜 Refactor

- *(ui)* Extract runtime config and reorganize admin/client modules - ([2748059](https://github.com/JQiue/yoin/commit/2748059dedb00ce4c6d48a0c3262fd9fd9b18c6c))
- Refactor the extractor and handler - ([48c8ea1](https://github.com/JQiue/yoin/commit/48c8ea1b26c41af4b21249da9d68e3bedd3ab4a7))
- Improved error handling for loading.env files - ([41286e5](https://github.com/JQiue/yoin/commit/41286e5ee0c6d5e63765f463022a42fc4fff0af9))
- Wrap AppState in Arc and use tokio Mutex - ([ca0e235](https://github.com/JQiue/yoin/commit/ca0e2357deedac7a8555538dcf15d82ec55189d4))
- Optimize existing code structure - ([75d2332](https://github.com/JQiue/yoin/commit/75d23327a1e073ae11e056e6817de70092a79386))
- Refactor repository and service layers to implement repository traits for Users, Sites, and Comments - ([bdb6e09](https://github.com/JQiue/yoin/commit/bdb6e09edd0562d3a184d825e324f05c4f88e60a))
- Replace rate_limit_cache with RateLimiter - ([c1494d0](https://github.com/JQiue/yoin/commit/c1494d0e370953860ce58debbb8909bc875aac7c))
- Rename url to website in models and related code - ([a30cc32](https://github.com/JQiue/yoin/commit/a30cc32c539901716d270c10145857384b977f2d))
- Refactor admin id and site config handling - ([5b0cae3](https://github.com/JQiue/yoin/commit/5b0cae34b73ac0fd4e4d5159f122135759f41a5f))
- Organize repository modules - ([4f6f9a0](https://github.com/JQiue/yoin/commit/4f6f9a09b59200628ac4f64d32ea17ceec5eb116))
- Refactor service module structure - ([ddff6a2](https://github.com/JQiue/yoin/commit/ddff6a253e0bb4b72bff70fa7a0c0a08cbfbd1d1))
- Refactor user check logic to require_admin - ([dbdca6d](https://github.com/JQiue/yoin/commit/dbdca6d39c7876a51fba35de39b470eae8a3fe08))
- Simplify authentication logic in RequireAuth and OptionnalAuth - ([e2b7a39](https://github.com/JQiue/yoin/commit/e2b7a39826c8639657b9cef54277086e22454af9))
- Update repository traits and methods for consistency - ([1b2d2b3](https://github.com/JQiue/yoin/commit/1b2d2b30957fd77feaa50a93261f35b597db22cf))
- Update module imports and enhance service structure - ([e829773](https://github.com/JQiue/yoin/commit/e8297739ec615d15c87c78a2f7fa8c61b33d89e3))
- Simplify comment update logic - ([265e5e9](https://github.com/JQiue/yoin/commit/265e5e9d4f73eea29e7d501a3840ed9f014b9361))
- Better readability and maintainability - ([5a958d5](https://github.com/JQiue/yoin/commit/5a958d5c8a3db4c018fc850bb9b1c8b17d51e781))
- Improve UI consistency - ([8472ace](https://github.com/JQiue/yoin/commit/8472ace9aff64477e3064b03eedab5768c194ac4))

### 🧪 Testing

- Add integration tests for auth, site, and comment flows - ([6e2962a](https://github.com/JQiue/yoin/commit/6e2962a042303e9b6b303781eeb759cb37e700cd))
- Sync integration tests with current implemented APIs - ([b791768](https://github.com/JQiue/yoin/commit/b79176817f377b281b4e7dde583064dc9d5a18dd))

### ⚙️ Miscellaneous Tasks

- Update dependencies - ([d44e74f](https://github.com/JQiue/yoin/commit/d44e74fd74d527554acd02f544fa2ae2d65036f2))
- Update Dockerfile and docker-compose for improved build process - ([60afb08](https://github.com/JQiue/yoin/commit/60afb08fe55d6e3fa6ab3113d2278861ad3ae500))
- Update dependencies in Cargo.toml and Cargo.lock for improved compatibility - ([bc8ad31](https://github.com/JQiue/yoin/commit/bc8ad3117e51a2a5f542e14f3a07a8a17d340ce9))
- Format code - ([3e885e9](https://github.com/JQiue/yoin/commit/3e885e925ed6c984c7c70abb55843e59f05fee6f))
- Update TypeScript configuration and paths in tsconfig.json - ([3ccbb5e](https://github.com/JQiue/yoin/commit/3ccbb5e2f00948aeb215059b83c59a033ffd433b))
