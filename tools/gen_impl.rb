indent = "    "
param1 = "T"
param2 = "B"
init = "impl_tuple_map!{\n" + indent + "TupleMap"
max_items = 16
(1..max_items).each { |i|
  impl = init
  name_reduced = (0...i - 1).map {|j| ("a".ord + j + 1).chr}.join(" ")
  names = (0...i).map { |j| ("a".ord + j).chr }.join(" ")
  names2 = (0...i).map { |j| ("a".ord + j).chr + '2'}.join(" ")
  items = (1..i).map { |_| 'Item'}.join(" ")
  p1 = (1..i).map { |_| param1}.join(" ")
  p2 = (1..i).map { |_| param2}.join(" ")
  impl += i.to_s + ",\n"
  impl += indent + name_reduced + ", \n"
  impl += indent + names + ", \n"
  impl += indent + names2 + ", \n"
  impl += indent + items + ",\n"
  impl += indent + p1 + ", \n"
  impl += indent + p2 + "\n}"
  puts impl
}

