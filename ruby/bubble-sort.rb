# frozen_string_literal: true

# Sorts a list using bubble sort alogithm
#
# @param t [Hash] elements to sort
def bubble_sort(t)
  ordered = false
  until ordered
    ordered = true
    (0..t.length - 2).each do |i|
      next unless t[i] > t[i + 1]

      t[i], t[i + 1] = t[i + 1], t[i]
      ordered = false
    end
  end
  t
end
