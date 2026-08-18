# REFACTOR CHECK LIST:

## ON WORKING:

- [ ] Appstate - devine state into seperate module for easy maintain - may need a docs anyway, because its quite hard to read every single file.
- [ ] App router - middleware (seem not really protected yet, so i will change something soon, like cookies, lax, some infra(domain) problems)

# DONE:

- [x] Config file parser
- [x] Error repsonse - may need to change in future but not the must.
- [x] Telemetry - log output config.

# MODULE REFACTORING LIST

- [ ] **Admin**
- [x] **Auth**
  - Removed password login from auth -> change to oauth system instead (currently only Google login is viable to use)
  - Sub state for module. Refactor JWT as well.
- [ ] **Cart**
- [ ] **Category**
- [ ] **Order**
- [ ] **Product**
- [ ] **User**
