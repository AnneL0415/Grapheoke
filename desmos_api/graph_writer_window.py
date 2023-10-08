def graph_fn(arr, xmin, xmax, ymin, ymax):
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
    output_file.write("\t\tlyric_calculator.controller.externalSetState(lyric_calculator.controller.getBlankState());\n\t")
    # output_file.write("\tlyric_calculator.controller.dispatch({type: \"set-axis-scale\", settings: {axis: \"y\", scale: \"linear\", limitLatex: {min: \"" + str(ymin) + "\",max: \"" + str(ymax) + "\"}}});\n")
    # output_file.write("\t\tlyric_calculator.controller.dispatch({type: \"set-axis-scale\", settings: {axis: \"x\", scale: \"linear\", limitLatex: {min: \"" + str(xmin) + "\",max: \"" + str(xmax) + "\"}}});")
    output_file.write("\n\t}\n\n")

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
        output_file.write("setTimeout(function() {\n")

        tab_repeater(j)
        output_file.write("clear_graph()\n")

        for function in element[1]:
            # print(repr(function))
            tab_repeater(j)
            output_file.write("lyric_calculator.setExpression({ id: 'line" + str(k) + "', latex: ")
            output_file.write(repr(function))
            output_file.write(", color: '#000000' });\n")
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

def music_fn(arr):
    # create file
    output_file = open("graph.html", "w")

    # set basic parameters
    output_file.write("<script src=\"calculator.js\"></script>\n")
    output_file.write("<div id=\"calculator\" style=\"width: auto; height: auto;\"></div>\n")
    output_file.write("<script>\n")
    output_file.write("\tvar elt = document.getElementById('calculator');\n")
    output_file.write("\tvar calculator = Desmos.GraphingCalculator(elt);\n")

    # write and format each function
    i = 1
    for fn in arr:
        output_file.write("\tcalculator.setExpression({ id: 'line" + str(i) + "', latex: ")
        output_file.write(repr(fn))
        output_file.write(", color: '#000000' });\n")
        i+=1
    
    # end file
    output_file.write("\tcalculator.openKeypad();\n")
    output_file.write("</script>")
