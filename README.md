# TESTCLI

testing dotenvy to read out of three different locations:
- working directory in which the cli is executed
- relative directory to which the cli is executed
- $HOME/.runcli/config as last resort if first two env files do not exist,
  enabling testcli to be used directory independent


the project_path within the env file should be absolute for scripts/.runrsenv and $HOME/.runcli/config.
.runrsenv can be relative.
