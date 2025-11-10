function geo_point(tag, timestamp, record)
    local lat = record["latitude"]
    local lon = record["longitude"]

    if lat ~= nil and lon ~= nil then
        record["location"] = lat .. "," .. lon
    end

    return 1, timestamp, record
end
