-module(count_rect_area).
-export([ main/0]).

calculate_rectangle_area(TopLeftX, TopLeftY, BottomRightX, BottomRightY) ->
    Width = BottomRightX - TopLeftX,
    Height = TopLeftY - BottomRightY,
    abs(Width * Height).

read_floats(FilePath) ->
    {ok, File} = file:open(FilePath, [read]),
    {ok, Floats} = read_floats_helper(File, []),
    file:close(File),
    Floats.

read_floats_helper(File, Acc) ->
    case file:read_line(File) of
        {ok, Line} ->
            FloatList = string:tokens(Line, " "),
            Floats = lists:map(fun list_to_floats/1, FloatList),
            read_floats_helper(File, Floats ++ Acc);
        eof ->
            {ok, lists:reverse(Acc)}
    end.

list_to_floats(Str) ->
    {Float, _Rest} = string:to_float(Str),
    Float.

unpack_floats([XTop, YTop, XBot, YBot]) ->
    {XTop, YTop, XBot, YBot}.

create_output_directory() ->
    case filelib:is_dir("output") of
        true -> ok;
        false -> filelib:make_dir("output")
    end.

main() ->
    io:format("Do you want to read coordinates from .txt file or console? \nType 'c' for console or 'f' for file\n"),
    case io:get_line("") of
        "f\n" ->
            io:format("Enter filename.\nIt should be located in folder ./input/\n"),
            FileNameTmp = io:get_line(""),
			FileName = string:strip(FileNameTmp, right, $\n),
			FilePath = "./input/" ++ FileName,
            Floats = read_floats(FilePath),
			{XTop, YTop, XBot, YBot} = unpack_floats(Floats),
			io:format("~nXTop: ~p~nYTop: ~p~nXBot: ~p~nYBot: ~p~n~n", [XTop, YTop, XBot, YBot]),
			Area = calculate_rectangle_area(XTop, YTop, XBot, YBot),
			io:format("Rectangle area: ~p~n~nThe result was saved in dir ./output/~s~n", [Area, FileName]),
			create_output_directory(), 
            {ok, OutputFile} = file:open("output/" ++ FileName, [write]),
            io:format(OutputFile, "~p", [Area]),
            file:close(OutputFile);
			
        "c\n" ->
		{ok, [XTop, YTop, XBot, YBot]} = io:fread("Enter coordinates with a fractional part in format:  x1 y1 x2 y2 \n", "~f ~f ~f ~f"),
			io:format("~nXTop: ~p~nYTop: ~p~nXBot: ~p~nYBot: ~p~n~n", [XTop, YTop, XBot, YBot]),
            Area = calculate_rectangle_area(XTop, YTop, XBot, YBot),
            io:format("Rectangle area: ~p~n", [Area]);
        _ ->
            io:format("No such command~n")
    end.
