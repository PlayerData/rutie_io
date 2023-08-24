# frozen_string_literal: true

require_relative "../rutie_io_example"

RSpec.describe RutieIOExample do
  it 'echos with StringIO' do
    string_io_in = StringIO.new("Hello World")
    string_io_out = StringIO.new

    RutieIOExample.echo(string_io_in, string_io_out)

    string_io_out.rewind
    expect(string_io_out.read).to eq "Hello World"
  end
end
