def graph_fn(arr):
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
        output_file.write("\tcalculator.setExpression({ id: 'line" + str(i) + "', latex: '")
        output_file.write(fn)
        output_file.write("', color: '#000000' });\n")
        i+=1
    
    # end file
    output_file.write("\tcalculator.openKeypad();\n")
    output_file.write("</script>")