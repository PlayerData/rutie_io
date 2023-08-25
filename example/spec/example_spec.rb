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

  it 'can read single char' do
      string_io_in = StringIO.new("Hello World")

      char = RutieIOExample.read_char(string_io_in)
      expect(char).to eq "H"
      char = RutieIOExample.read_char(string_io_in)
      expect(char).to eq "e"

      string_io_in.rewind
      char = RutieIOExample.read_char(string_io_in)
      expect(char).to eq "H"

      string_io_in.seek(20)
      char = RutieIOExample.read_char(string_io_in)
      expect(char).to eq ""
  end
end
