# dummy_autopilot
It's a very simple tool for end to end tests of the new cow protocol driver.

It queries the current auction from the actual `autopilot`, passes that to the `/solve` endpoint of the local driver and calls `/execute` for every solver that successfully computed a result.

The queried solvers are: `otex`, `quasimodo`, `seasolver`, `plm`, `stakecapital`, `atlas`, `re7`, `jeffreyliang`
