name: Bug report
about: Report a bug or issue in the project
title: ""
labels: bug
assignees: ""

body:
  - type: markdown
    attributes:
      value: |
        Thanks for reporting a bug. Please fill out the information below so we can investigate it more quickly.
  - type: textarea
    id: description
    attributes:
      label: Bug description
      description: Describe the issue clearly.
      placeholder: Example: When ... then ...
    validations:
      required: true
  - type: textarea
    id: steps
    attributes:
      label: Steps to reproduce
      description: List the steps needed to reproduce the issue.
      placeholder: 1. ...
    validations:
      required: true
  - type: textarea
    id: expected
    attributes:
      label: Expected result
      description: Describe what you expected to happen.
    validations:
      required: true
  - type: textarea
    id: actual
    attributes:
      label: Actual result
      description: Describe the actual behavior you observed.
    validations:
      required: true
