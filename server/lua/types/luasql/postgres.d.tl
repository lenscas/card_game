global record Sql
    postgres: function():Driver
    record Driver
        connect:function(Driver,string):Connection
    end
    record Connection
        setautocommit:function(Connection,boolean):boolean
        escape: function(Connection,string):string
        execute:function(Connection,string):boolean,any
        commit: function(Connection)
    end
end
return Sql