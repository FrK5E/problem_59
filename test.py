characters=[]
with open( 'test_text.txt', 'r' ) as f: 
    data = f.read()
    for d in data: 
        characters.append( str( ord(d) ) ) 

print( ",".join(characters) )

