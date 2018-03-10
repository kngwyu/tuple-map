indent = "    "
param1 = "T"
param2 = "B"
init = "impl_tuple_map!{\n" + indent + "TupleMap"
max_items = 16
(1..max_items).each { |i|
  impl = init
  impl += i.to_s + ",\n"
  names = (0...i).map { |j| ("a".ord + j).chr }.join(" ")
  items = (1..i).map { |_| 'Item'}.join(" ")
  p1 = (1..i).map { |_| param1}.join(" ")
  p2 = (1..i).map { |_| param2}.join(" ")
  impl += indent + names + ", \n"
  impl += indent + items + ",\n"
  impl += indent + p1 + ", \n"
  impl += indent + p2 + "\n}"
  puts impl
}
