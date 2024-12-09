#Tasks

> **Note**: The obsidian dataview plugin is required for viewing live task progress.  
> Also, the minimal theme is required for the custom checkboxes
# Overdue
```dataview
TASK
FROM #Tasks
WHERE due <= date(today) and status = " "
```

# Left
```dataview
TASK
FROM #Tasks
WHERE due > date(today) and status = " "
```

# All

- [x] Complete planning  [due:: 2024-10-29]
- [x] Parse all reference PDFs  [due:: 2024-11-05]
- [x] Find common instructions  [due:: 2024-11-12]
- [x] Research a hello world and the instructions used by it  [due:: 2024-11-19]
- [-] Define an ISA based on the findings  [due:: 2024-11-26]
- [-] Find a suitable target for a proof of concept  [due:: 2024-12-03]
- [-] Implement a proof of concept  [due:: 2024-12-10]
- [-] Test the proof of concept and extract conclusions  [due:: 2024-12-17]
- [ ] Implement a working prototype  [due:: 2024-12-17]
- [ ] Obtain conclusions  [due:: 2024-12-17]
