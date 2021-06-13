#!/usr/bin/env ruby

dir_path = ARGV[0]

Dir.foreach(dir_path) do |f|
  fn = File.join(dir_path, f)
  File.delete(fn) if f != '.' && f != '..'
end
