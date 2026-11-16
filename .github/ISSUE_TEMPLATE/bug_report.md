name: Bug report
about: Báo cáo lỗi hoặc sự cố trong dự án
title: ""
labels: bug
assignees: ""

body:
  - type: markdown
    attributes:
      value: |
        Cảm ơn bạn đã báo cáo lỗi. Hãy điền thông tin dưới đây để giúp chúng tôi khắc phục nhanh hơn.
  - type: textarea
    id: description
    attributes:
      label: Mô tả lỗi
      description: Mô tả rõ ràng về vấn đề bạn gặp phải.
      placeholder: Ví dụ: Khi ... thì ...
    validations:
      required: true
  - type: textarea
    id: steps
    attributes:
      label: Các bước để tái hiện
      description: Liệt kê các bước để lặp lại lỗi.
      placeholder: 1. ...
    validations:
      required: true
  - type: textarea
    id: expected
    attributes:
      label: Kết quả mong đợi
      description: Mô tả điều bạn mong đợi xảy ra.
    validations:
      required: true
  - type: textarea
    id: actual
    attributes:
      label: Kết quả thực tế
      description: Mô tả hiện tượng thực tế.
    validations:
      required: true
