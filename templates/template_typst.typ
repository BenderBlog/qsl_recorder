#let header-cell(ch, en) = {
  block(inset: (y: 4pt), stack(dir: ttb, spacing: 2pt, align(center, ch), align(center, text(8pt, en))))
}
#let get-optional(dict, key) = {
  if dict.keys().contains(key) {
    dict.at(key)
  } else { "" }
}

#set text(font: "Source Han Serif")
#set page(paper: "a4", flipped: true, margin: (x: 1.27cm, y: 1.27cm))

#let id = 0
#let xubiao = state("xubiao")
#set table(stroke: (x, y) => {
  if y == 0 { none } else { 1pt }
})
#show table: it => xubiao.update(false) + it

#table(
  columns: (auto, auto, auto, auto, 1.2fr, 1.2fr, 0.8fr, 0.7fr, 0.7fr, 2fr, 1.5fr, 1.5fr, 0.7fr, 0.7fr, 2.5fr),
  align: center + horizon,
  stroke: 0.5pt,
  table.header(
    table.cell(rowspan: 2, text(10pt)[序号]),
    table.cell(rowspan: 2, header-cell[日期][DATE]),
    table.cell(rowspan: 2, header-cell[时间][TIME]),
    table.cell(rowspan: 2, header-cell[天气][WX]),
    table.cell(rowspan: 2, header-cell[呼号][CALLSIGN]),
    table.cell(rowspan: 2, header-cell[频率][MHz]),
    table.cell(rowspan: 2, header-cell[模式][MODE]),
    table.cell(colspan: 2, header-cell[信号报告][RST]),
    table.cell(rowspan: 2, header-cell[电台位置][QTH]),
    table.cell(rowspan: 2, header-cell[设备][RIG]),
    table.cell(rowspan: 2, header-cell[天线][ANT]),
    table.cell(colspan: 2, header-cell[功率][PWR]),
    table.cell(rowspan: 2, header-cell[备注][RMKS]),
    [己方],
    [对方],
    [己方],
    [对方],
  ),
  ..for log in log_data {
    id += 1;
    (
      str(id),
      log.date,
      log.time,
      [],
      log.call_number,
      get-optional(log, "freq"),
      log.mode,
      get-optional(log, "rst_me"),
      get-optional(log, "rst_counterpart"),
      get-optional(log, "qth_counterpart"),
      get-optional(log, "rig_me"),
      get-optional(log, "ant_me"),
      get-optional(log, "watt_me"),
      get-optional(log, "watt_counterpart"),
      get-optional(log, "note"),
    )
  },
)