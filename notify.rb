#!/bin/env ruby

require 'ffi'

module NotifyRust
  extend FFI::Library
  ffi_lib 'target/debug/libnotify_rust.so'
  attach_function :get_capabilities, [], :string
  attach_function :notify_show, [:string], :void
end

def get_capabilities
  NotifyRust::get_capabilities().split ?,
end



NotifyRust::notify_show("foo")

