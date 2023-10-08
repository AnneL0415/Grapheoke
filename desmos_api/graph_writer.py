def graph_fn(arr):
    # create file
    output_file = open("graph.html", "w")

    #source
    output_file.write("<script src=\"calculator.js\"></script>\n")

    # setup lyric graph
    output_file.write("<div id=\"calculator\" style=\"width: 100%; height: 50%;\"></div>\n")
    output_file.write("<script>\n")
    output_file.write("\tvar lyric_elt = document.getElementById('calculator');\n")
    output_file.write("\tvar lyric_calculator = Desmos.GraphingCalculator(lyric_elt);\n\n")

    # clear graph function
    output_file.write("\tfunction clear_graph() {\n")
    output_file.write("\t\tlyric_calculator.controller.externalSetState(lyric_calculator.controller.getBlankState());\n\t}\n\n")

    # tab repeater
    def tab_repeater(num):
        while (num >= 1):
            output_file.write("\t")
            num-=1

    # write and format each function
    i = 1
    j = 2
    k = 1
    for element in arr:
        tab_repeater(i)
        output_file.write("setTimeOut(function() {\n")

        tab_repeater(j)
        output_file.write("clear_graph()\n")

        for function in element[1]:
            tab_repeater(j)
            output_file.write("lyric_calculator.setExpression({ id: 'line" + str(k) + "', latex: '")
            output_file.write(function)
            output_file.write("', color: '#000000' });\n")
            k+=1
        
        j+=1
        i+=1

    # close all functions 
    while i >= 2:
        tab_repeater(i-1)
        output_file.write("}, " + str(arr[i-2][0]) + ")")
        i-=1
        if i == 1:
            output_file.write(";\n")
        else:
            output_file.write("\n")
    
    # end lyric graph
    output_file.write("</script>\n\n")

    # ------

    # setup music graph
    output_file.write("<div id=\"calculator\" style=\"width: 100%; height: 50%;\"></div>\n")
    output_file.write("<script>\n")
    output_file.write("\tvar music_elt = document.getElementById('calculator');\n")
    output_file.write("\tvar music_calculator = Desmos.GraphingCalculator(music_elt);\n\n")

    # end music graph
    output_file.write("</script>")

# tests
# arr = [(5.32, ["x^2", "x^3", "x^4", "x^5"]), (5.32, ["2", "3", "4", "5"]), (5.32, ["0.5x", "x", "1.5x", "2x"])]
# graph_fn(arr)