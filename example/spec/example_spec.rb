# frozen_string_literal: true

require "tempfile"

require_relative "../rutie_io_example"

RSpec.describe RutieIOExample do
  it 'echos with StringIO' do
    string_io_in = StringIO.new("Hello World")
    string_io_out = StringIO.new

    RutieIOExample.echo(string_io_in, string_io_out)

    string_io_out.rewind
    expect(string_io_out.read).to eq "Hello World"
  end

  it 'echoes with File' do
    Tempfile.create do |file_in|
      file_in.write("Hello World")
      file_in.rewind

      Tempfile.create do |file_out|
        RutieIOExample.echo(file_in, file_out)

        file_out.rewind
        expect(file_out.read).to eq "Hello World"
      end
    end
  end
end
